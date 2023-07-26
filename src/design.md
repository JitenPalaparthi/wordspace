# design

## api endpoints

```
1. To add a word     post    http://<baseurl:port>/v1/public/words
2. To get a word     get     http://<baseurl:port>/v1/public/words/:word
3. To update a word  put     http://<baseurl:port>/v1/public/words/:word
4. To delete aword   delete  http://<baseurl:port>/v1/public/words/:word
5. To fetch  all     get     http://<baseurl:port>/v1/public/words/all
```

## flow

- This is a restful api
- To add a word, user has to use post endpoint (1.). 
- The data has to be stored as a file with word name
- A file gets created. The name of the file is the name of the word.
- All the data that gets strored is seriliased to json format


