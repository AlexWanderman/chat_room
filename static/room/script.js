// Filler message for debug and preview
function filler_message() {
  const lorem = [
    "Irure est nisi mollit enim laboris excepturi anim dolorem minim dicta animi nemo provident nulla.",
    "Excepteur beatae numquam ut ea nulla aute irure inventore inventore nemo dolor ipsam veniam amet.",
    "Deleniti nisi numquam magnam sed magnam ut enim vitae ducimus excepteur irure qui irure enim.",
    "Similique accusamus nostrud nemo nemo anim occaecat sequi accusamus atque non occaecat.",
    "Voluptatum nostrud reprehenderit veritatis et in ab nulla fugiat enim neque similique.",
    "Commodo illo cillum et nesciunt dolore mollit amet nemo sed ratione dolore lorem.",
    "Obcaecati excepteur amet neque enim nulla quas quia ipsum deserunt odit tempora.",
    "Labore ipsum sequi neque in deleniti quia anim quos ab ducimus quia.",
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    "Laboris quia nulla fugiat veritatis incididunt.",
    "Aliquip ipsum sint aut ullamco lorem similique.",
    "Ipsum quaerat animi dolor tempora voluptatum.",
    "Cillum velit officia molestias dolores.",
    "Et sit corrupti porro elit.",
  ];
  const random = Math.floor(Math.random() * lorem.length);
  return lorem[random];
}

let var_member = null;
let var_last_sender_uuid = null;
let var_last_receiver_uuid = null;
let var_members = new Map();

const el_menu_dialog = document.getElementById("menu-dialog");
const el_menu_you = document.getElementById("menu-you");
const el_menu_members = document.getElementById("menu-members");
const el_menu_close = document.getElementById("menu-close");
const el_header_quit = document.getElementById("header-quit");
const el_header_menu = document.getElementById("header-menu");
const el_messages_list = document.getElementById("messages-list");
const el_message_form = document.getElementById("message-form");
const el_message_form_text = document.getElementById("message-form-text");
const el_message_form_submit = document.getElementById("message-form-submit");

// WebSocket
// On connection to WS chat endpoint server expects client
// to have auth cookie with token for this room
const room_uuid = document.body.dataset.roomUuid;

let ws;
let var_retry_attempts = 0;
const var_retry_interval = 1_000;

function open_connection() {
  // Avoid opening another socket while one is connecting or open
  if (ws) {
    console.log("WS already exist");
    return;
  }

  // WS connection
  console.log("WS opening connection");
  ws = new WebSocket("/ws/room/" + room_uuid);

  ws.addEventListener("open", (e) => {
    console.log("WS open", e);
    var_retry_attempts = 0;
  });

  ws.addEventListener("error", (e) => {
    console.log("WS error", e);
  });

  ws.addEventListener("close", (e) => {
    console.log("WS close", e);

    // Remove WS
    ws = null;

    // Check if status code is ok
    if (e.code === 1000) {
      alert("Connection was closed");
      system_message("Connection was closed");
      window.location.href = "/";
      return;
    }

    // Schedule reconnect
    if (var_retry_attempts > 10) {
      alert("Connection failed");
      system_message("Connection failed");
      return;
    }

    var_retry_attempts += 1;

    setTimeout(() => {
      console.log("WS reconnect attempt " + var_retry_attempts);
      open_connection();
    }, var_retry_interval);
  });

  ws.addEventListener("message", (e) => {
    const message = JSON.parse(e.data);
    console.log("IN", message);

    // member: MemberList
    // message.member.uuid
    // message.member.name
    // message.member.is_online
    // message.member.last_seen

    switch (message.type) {
      case "SyncIdentity": {
        el_menu_you.innerText = "You: " + message.member.name;
        el_menu_you.title = message.member.uuid;
        var_member = message.member;
        break;
      }

      case "SyncMembers": {
        var_members.clear();

        message.members.forEach((member) => {
          var_members.set(member.uuid, member);
        });

        list_members();
        break;
      }

      case "MemberState": {
        if (message.member.is_online) {
          var_members.set(message.member.uuid, message.member);
          member_connected(message.member.uuid);
        } else {
          const member = var_members.get(message.member.uuid);
          member.is_online = false;
          member_disconnected(message.member.uuid);
        }
        list_members();
        break;
      }

      case "PublicMessage": {
        member_message(message.message, message.sender);
        break;
      }

      case "PrivateMessage": {
        member_message(message.message, message.sender, message.receiver);
        break;
      }

      case "BadMessage": {
        alert(message.reason);
        break;
      }

      default: {
        system_message("Unknown server message");
        break;
      }
    }
  });
}

// Must open initial WS connection
open_connection();

// Schedule next refresh request
async function refresh_token_timer() {
  console.log("WS refresh attempt");

  // Try to refresh token
  const response = await fetch("/api/refresh", {
    method: "POST",
    body: JSON.stringify({ room_uuid }),
    headers: { "Content-type": "application/json; charset=UTF-8" },
  });

  console.log("Refreshing token", response);

  // If it fails - redirect to index page
  if (response.status !== 200) {
    alert("Token refresh failed");
    window.location.href = "/";
  }

  // Schedule next attempt
  setTimeout(() => refresh_token_timer(), 10 * 60 * 1000);
}

// Start refresh timer
await refresh_token_timer();

el_menu_close.addEventListener("click", () => {
  el_menu_dialog.open = false;
});

el_header_quit.addEventListener("click", () => {
  window.location.href = "/";
});

el_header_menu.addEventListener("click", () => {
  el_menu_dialog.open = true;
});

el_message_form.addEventListener("submit", (e) => {
  e.preventDefault();

  let message = el_message_form_text.value.trim();
  if (message.length === 0) message = filler_message(); // Filler message
  if (!message || !ws) return;

  let keep_receiver = null;
  let value;

  // Request list of current members
  if (message.startsWith("/members")) {
    value = { type: "SyncMembers" };
  }
  // Private message
  else if (message.startsWith("@")) {
    const delimiter = message.indexOf(" ");
    if (delimiter === -1) return;

    const receiver_name = message.substring(1, delimiter).trim();
    let private_message = message.substring(delimiter + 1).trim();

    if (private_message.length === 1) private_message = filler_message(); // Filler message

    let receiver_uuid = null;

    for (const [uuid, member] of var_members) {
      if (member.name === receiver_name) {
        receiver_uuid = uuid;
        break;
      }
    }

    if (!receiver_uuid) return;

    value = { type: "PrivateMessage", receiver: receiver_uuid, message: private_message };
    keep_receiver = receiver_name;
  }
  // Public message
  else {
    value = { type: "PublicMessage", message };
  }

  // Send message
  console.log("OUT", value);
  ws.send(JSON.stringify(value));

  // Reset message input
  el_message_form_text.value = keep_receiver ? "@" + keep_receiver + " " : "";
  el_message_form_text.focus();
});

function list_members() {
  el_menu_members.replaceChildren();

  const header_div = document.createElement("div");
  header_div.textContent = "Members";

  el_menu_members.appendChild(header_div);

  for (const [uuid, member] of var_members) {
    const online = member.is_online ? " (online)" : " (offline)";

    const member_div = document.createElement("div");
    member_div.textContent = member.name + online;
    member_div.title = member.uuid;

    el_menu_members.appendChild(member_div);
  }
}

function member_connected(member_uuid) {
  const member = var_members.get(member_uuid);

  const content_div = document.createElement("div");
  content_div.textContent = member.name + " connected";
  content_div.className = "member-announce";

  el_messages_list.appendChild(content_div);
  el_messages_list.scrollTop = el_messages_list.scrollHeight;
  var_last_sender_uuid = null;
  var_last_receiver_uuid = null;
}

function member_disconnected(member_uuid) {
  const member = var_members.get(member_uuid);

  const content_div = document.createElement("div");
  content_div.textContent = member.name + " disconnected";
  content_div.className = "member-announce";

  el_messages_list.appendChild(content_div);
  el_messages_list.scrollTop = el_messages_list.scrollHeight;
  var_last_sender_uuid = null;
  var_last_receiver_uuid = null;
}

function member_message(message, sender_uuid, receiver_uuid = null) {
  // Message content
  const content_div = document.createElement("div");
  content_div.textContent = message;
  content_div.className = "message-content";

  // Same sender and receiver - continue message
  if (var_last_sender_uuid === sender_uuid && var_last_receiver_uuid === receiver_uuid) {
    const last_message = el_messages_list.lastChild;
    last_message.appendChild(content_div);
  }
  // Else - start add new message
  else {
    // Message header
    const header_div = document.createElement("div");
    header_div.className = "message-header";

    // Sender
    const sender = var_members.get(sender_uuid);

    // Sender name
    const sender_span = document.createElement("span");
    sender_span.textContent = sender.name;
    sender_span.title = sender.uuid;

    // Sender is self or other
    if (var_member.uuid === sender_uuid) {
      sender_span.className = "message-self";
    } else {
      sender_span.className = "message-other";
      sender_span.addEventListener("click", () => {
        el_message_form_text.value = "@" + sender.name + " ";
        el_message_form_text.focus();
      });
    }

    // Append sender to the header
    header_div.appendChild(sender_span);

    // Receiver name if not none
    if (receiver_uuid) {
      // Spacer
      const spacer_span = document.createElement("span");
      spacer_span.textContent = "to";

      // Receiver
      const receiver = var_members.get(receiver_uuid);

      // Sender name
      const receiver_span = document.createElement("span");
      receiver_span.textContent = receiver.name;
      receiver_span.title = receiver.uuid;

      // Receiver is self or others
      if (var_member.uuid === receiver_uuid) {
        receiver_span.className = "message-self";
      } else {
        receiver_span.className = "message-other";
        receiver_span.addEventListener("click", () => {
          el_message_form_text.value = "@" + receiver.name + " ";
          el_message_form_text.focus();
        });
      }

      // Add spacer and receiver to the header
      header_div.appendChild(spacer_span);
      header_div.appendChild(receiver_span);
    }

    // Message
    const new_message_div = document.createElement("div");
    new_message_div.className = "message";
    new_message_div.appendChild(header_div);
    new_message_div.appendChild(content_div);

    // Add message to the list
    el_messages_list.appendChild(new_message_div);
  }

  el_messages_list.scrollTop = el_messages_list.scrollHeight;
  var_last_sender_uuid = sender_uuid;
  var_last_receiver_uuid = receiver_uuid;
}

function system_message(message) {
  const content_div = document.createElement("div");
  content_div.textContent = message;
  content_div.className = "message";

  el_messages_list.appendChild(content_div);
  el_messages_list.scrollTop = el_messages_list.scrollHeight;
  var_last_sender_uuid = null;
  var_last_receiver_uuid = null;
}
