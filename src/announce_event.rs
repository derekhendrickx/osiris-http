use peer::Peer;
use torrents::Torrents;
use announce_request::AnnounceRequest;

// TODO: Announce events (https://www.blogsolute.com/what-is-torrent-tracker-how-it-works-detail/20187/):
// 
// started: The first request to the tracker must include the event key with this value.
// stopped: Must be sent to the tracker if the client is shutting down gracefully.
// completed: Must be sent to the tracker when the download completes. However, must not be sent if the download was already 100% complete when the client started. Presumably, this is to allow the tracker to increment the “completed downloads” metric based solely on this event.

#[derive(Debug)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    None,
}

impl AnnounceEvent {
    pub fn handle(&self, announce_request: &AnnounceRequest, torrents: &mut Torrents) {
        let info_hash = announce_request.get_info_hash();

        match self {
            AnnounceEvent::Started => {
                let peer = Peer::new(&announce_request);

                torrents.add_torrent(info_hash);
                torrents.add_peer(info_hash, &peer);
            },
            AnnounceEvent::Stopped => {
                torrents.remove_peer(info_hash, announce_request.get_peer_id());
            },
            AnnounceEvent::Completed => {
                let data = (announce_request.get_uploaded(), announce_request.get_downloaded(), announce_request.get_left());
                torrents.update_peer(info_hash, announce_request.get_peer_id(), data);
            },
            AnnounceEvent::None => {
                let data = (announce_request.get_uploaded(), announce_request.get_downloaded(), announce_request.get_left());
                torrents.update_peer(info_hash, announce_request.get_peer_id(), data);
            },
        }

        torrents.show_torrents();
    }
}
