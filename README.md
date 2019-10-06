# filmhouse_prime
find past filmhouse films on Amazon Prime

## Context

The [Filmhouse Cinema in Edinburgh](https://www.filmhousecinema.com) is a local cinema which shows
a really good selection of films. Over the years myself and others have used it as a proxy for
quality films to find and watch later; there was a video rental shop near my house which had a wall
of DVD's marked "Filmhouse Films".

I can't always see these quality films when they come out first, and I have an Amazon Prime
subscription, so the question was: can I replicate the video rental shop experience on Amazon Prime?

## October 2019 status

After a days worth of hacking:
- `filmhouse_scraper`: can successfully extract a list of films available on
  https://www.filmhousecinema.com/whats-on/
  - `films.json` contains all those from past year
- `amazon_searcher`: is able to use amazon site search for Amazon Prime films
  but quickly gets blocked as a robot. It also returns a bunch of false
  positives when other search results come back as it doesn't cross-check
  the matches with the film name.

I looked into other alternatives for searching:
- Amazon Product Advertising API: https://docs.aws.amazon.com/AWSECommerceService/latest/DG/becomingDev.html
  - To use this I need to "Have completed 3 qualifying sales in 180 days."; I'm not exactly sure what this means
- Streamzui: https://rapidapi.com/Streamzui/api/streamzui-amazon-prime-video-search?endpoint=5b6c1397e4b0aa2f76d201e1
  - This is an unofficial API and requires money to test, but looks like it may work

I'd prefer to use the official Amazon API's if possible, as it'd likely give me a cleaner result, and it is more
likely to stick around.

However, I want to work on other stuff now, and the list of films from last year is valuable in itself, so
pausing this for now.