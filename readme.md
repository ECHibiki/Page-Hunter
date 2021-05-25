# Page-Hunter

##About
A CLI application to search through page listings and return unique domain names<br/>

## usage

```./main url="https://www.bing.com/search?q=kissu&first=#d" stride=10 start=0 max=-1```
Will use the URL and replace #d with start going up by stride. When it hits max the program will terminate.<br/>
Case of start=-1: Will run infinitely adding to the SQLite database forever, or until the program reaches a terminating case(403, 404, duplicate page contents).<br/>
Defaults: stride=1 start=1 max=10
Required: url with a #d for pages
