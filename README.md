## twitter-media-archiver

this is mainly just a personal project to download media from my tweets. maybe
it might help someone else, too, who knows.

for downloading your tweet images, just run the binary on an extracted archive:

    twitter-media-archiver archive/

it'll output what files have been downloaded/skip ones that already exist.

video download not supported since the archive doesn't store a reference to the
video/gif and I don't want to bother adding in Twitter scraping functionality.
you have to use something like `youtube-dl` (which requires the tweet to be
public). you can grab a list of video tweets and use `youtube-dl` like:

    twitter-media-archiver --videos archive/ | xargs youtube-dl -o "%(uploader_id)s.%(id)s.%(ext)s"
