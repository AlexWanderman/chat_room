# Committed

- Reading address from environment variables
- Fixed readme file

# ToFix

- Rooms can be created without joining in and that will make them linger until someone joins and leaves properly

# ToDo

- Client:
  - On form submit error move focus to the problematic field
  - Interactive field validation
- Backend:
  - Unique member and room names
  - Reserve names for duration of token
  - Write auto tests
  - Pressing "Join" should send to the room if there is the token already
  - Improve logging
- About page:
  - Write proper page

# MaybeLater

- Host interface
  - Login as host for moderation
  - Ability to remove rooms
  - Announce server restart or shutdown
  - Server announcements
- http2 support
- Better auto scroll
- Invite links
- Default global chat that stays forever
- Rate limits
  - Password attempts
  - Creating rooms
  - Messages
- Member limits
- Mute or block another participant locally
- Validate and check receiver name from clients private message
- Better message length counting with support for emojis and etc
- Link detection and clickable URLs
- Room sorting

# MaybeNever

- Room owner
  - Change room name and description
  - Lock a room so no new users can join
  - Close a room
  - Remove users from a room
- Message timestamps
- Store n last messages
- Auto remove rooms after last message passed too long
- Mark rooms as active, quiet, or full
- Client parse messages for mentions
- Pack JSON as bytes
- Accessibility
- Different locales
