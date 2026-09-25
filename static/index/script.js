const el_create_room_dialog = document.getElementById("create-room-dialog");
const el_form = document.getElementById("form");
const el_form_name = document.getElementById("form-name");
const el_form_description = document.getElementById("form-description");
const el_form_is_listed = document.getElementById("form-is-listed");
const el_form_has_password = document.getElementById("form-has-password");
const el_form_password_label = document.getElementById("from-password-label");
const el_form_password = document.getElementById("form-password");
const el_form_member_name = document.getElementById("form-member-name");
const el_submit_btn = document.getElementById("submit-btn");
const el_close_btn = document.getElementById("close-btn");

const el_join_room_dialog = document.getElementById("join-room-dialog");
const el_join_room_name = document.getElementById("join-room-name");
const el_join_room_description = document.getElementById("join-room-description");
const el_join_form = document.getElementById("join-form");
const el_join_form_room_uuid = document.getElementById("join-form-room-uuid");
const el_join_form_member_name = document.getElementById("join-form-member-name");
const el_join_form_password_label = document.getElementById("join-form-password-label");
const el_join_form_password = document.getElementById("join-form-password");
const el_join_submit_btn = document.getElementById("join-submit-btn");
const el_join_close_btn = document.getElementById("join-close-btn");

const el_open_btn = document.getElementById("open-btn");
const el_refresh_btn = document.getElementById("refresh-btn");
const el_room_list = document.getElementById("room-list");

el_form.addEventListener("submit", async (e) => {
  e.preventDefault();

  const value = {
    name: el_form_name.value,
    description: el_form_description.value,
    is_listed: el_form_is_listed.checked ? true : false,
    password: el_form_has_password.checked ? el_form_password.value : null,
    member_name: el_form_member_name.value,
  };
  console.log("Create room request", value);

  const response = await fetch("/api/room", {
    method: "POST",
    body: JSON.stringify(value),
    headers: { "Content-type": "application/json; charset=UTF-8" },
  });
  console.log("Create room response", response);

  if (response.ok) {
    const room_uuid = await response.text();
    window.location.href = "/room/" + room_uuid;
  } else {
    alert(await response.text());
  }
});

el_form_has_password.addEventListener("click", () => {
  const display = el_form_has_password.checked ? "block" : "none";
  el_form_password_label.style.display = display;
  el_form_password.style.display = display;
});

el_close_btn.addEventListener("click", () => {
  el_create_room_dialog.open = false;
});

el_join_form.addEventListener("submit", async (e) => {
  e.preventDefault();

  const password = el_join_form_password.value;
  const value = {
    room_uuid: el_join_form_room_uuid.value,
    room_password: password === "" ? null : password,
    member_name: el_join_form_member_name.value,
  };
  console.log("Join room request", value);

  const response = await fetch("/api/auth", {
    method: "POST",
    body: JSON.stringify(value),
    headers: { "Content-type": "application/json; charset=UTF-8" },
  });
  console.log("Join room response", response);

  if (response.ok) {
    window.location.href = "/room/" + value.room_uuid;
  } else {
    alert(await response.text());
  }
});

el_join_close_btn.addEventListener("click", () => {
  el_join_room_dialog.open = false;
});

el_open_btn.addEventListener("click", () => {
  el_create_room_dialog.open = true;
});

el_refresh_btn.addEventListener("click", async () => {
  const response = await fetch("/api/room");
  const json = await response.json();

  console.log(json);

  if (response.status == 200) {
    el_room_list.replaceChildren();

    // There are no rooms yet
    if (json.length === 0) {
      const no_rooms_div = document.createElement("div");
      no_rooms_div.innerText = "Refresh or create new room";
      no_rooms_div.className = "no-rooms";
      el_room_list.appendChild(no_rooms_div);
    }

    for (const room of json) {
      // Name
      const name_span = document.createElement("span");
      name_span.innerText = room.name;
      name_span.title = room.uuid;
      name_span.className = "room-name";

      // Members
      const members_span = document.createElement("span");
      members_span.innerText = room.online + " / " + room.total + " online";
      members_span.className = "room-members";

      // Join
      const join_btn = document.createElement("button");
      join_btn.innerText = "Join " + (room.has_password ? "(password)" : "(open)");
      join_btn.className = "room-join";
      join_btn.addEventListener("click", () => join_room(room.uuid, room.name, room.description, room.has_password));

      // Title div
      const header_div = document.createElement("div");
      header_div.className = "room-header";
      header_div.appendChild(name_span);
      header_div.appendChild(members_span);
      header_div.appendChild(join_btn);

      // Description div
      const description_div = document.createElement("div");
      description_div.innerText = room.description;
      description_div.className = "room-description";

      // Room div
      const room_div = document.createElement("div");
      room_div.className = "room";
      room_div.appendChild(header_div);
      room_div.appendChild(description_div);

      // Add room to the list
      el_room_list.appendChild(room_div);
    }
  }
});

function join_room(uuid, name, description, has_password) {
  const display = has_password ? "block" : "none";

  el_join_room_dialog.open = true;

  el_join_room_name.innerText = name;
  el_join_room_description.innerText = description;

  el_join_form_room_uuid.value = uuid;
  el_join_form_password_label.style.display = display;
  el_join_form_password.style.display = display;
}

// Export join_room to use on the page
window.join_room = join_room;
