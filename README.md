## twitter-media-archiver

this is mainly just a personal project to download media from my tweets. maybe
it might help someone else, too, who knows.

video download not supported since the archive doesn't store a reference to the
video/gif and I don't want to bother adding in Twitter scraping functionality.
you have to use something like `youtube-dl` (which requires the tweet to be
public). you can grab a list of video tweets and use `youtube-dl` like:

    twitter-media-archiver --videos | xargs youtube-dl -o "%(uploader_id)s.%(id)s.%(ext)s"
