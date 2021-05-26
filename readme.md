# Page-Hunter

## About
A single threaded CLI application to search through page listings and return unique domain names<br/>

### Runtime
Program intializes SQLite DB, begins a loop making requests(single threaded to reduce chance of rate limits and make them easier to add in), parse the resulting html body and extract URLS, add unique domains into the database. <br/>
On duplicate page data or a 4xx response code the program will end, outputting the termination reason into the console.

## Usage

```./main url=https://www.bing.com/search?q=kissu&first=#d db=kissu.db stride=10 start=0 max=-1```<br/>
Will use the URL and replace #d with start going up by stride. When it hits max the program will terminate.<br/>
Case of start=-1: Will run infinitely adding to the SQLite database forever, or until the program reaches a terminating case(403, 404, duplicate page contents).<br/>
Defaults: stride=1, start=1, max=10<br/>
Required: url with a #d for pages , a db to be written to<br/>

## Rust Dependencies

SQLite, Reqwest, Scraper, Url
