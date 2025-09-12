
## Functions

- logseq에 포함된 `page` node title을 리스트로 표시할 수 있다
 - page title 을 표시한다
 - 만약 page에 tag가 있다면 tag도 표시한다
- 사용자가 선택한 `page` node를 logseq로 open한다

## How

logseq graph의 정보를 얻기 위해 logseq cli를 subprocess로 실행한 후 edn 형식의 stdout결과를 사용한다

```
$ npx @logseq/cli -h
```

쿼리예시

> plugin config 로 어떤 graphq를 사용할지 얻어야 한다. 일단 `illef2`로 하드코딩한다

```bash
$ # page title 과 uuid를 출력
$ npx @logseq/cli query illef2 '[:find (pull ?b [:block/tags :block/uuid :block/title]) :where [?tag :block/name "page"] [?b :block/tags ?tag]]'
[{:block/tags [{:db/id 136} {:db/id 387}],
  :block/title "'될놈될'은 진실인가?",
  :block/uuid #uuid "67c56741-df35-408b-a4d8-df08ebcd030e"}
 {:block/tags [{:db/id 136} {:db/id 212}],
  :block/title "삶",
  :block/uuid #uuid "6821bf0d-3ac7-42cc-a5ba-5b742b809633"}
 {:block/tags [{:db/id 136} {:db/id 394}],
  :block/title "김우창",
  :block/uuid #uuid "68195db4-0a0c-419a-a0b3-e0ee9ab4bff1"}
 {:block/tags [{:db/id 136} {:db/id 394}],
  :block/title "에딩턴",
  :block/uuid #uuid "67c84405-9c93-4b3a-9e72-40349d7a1ed0"}
 {:block/tags [{:db/id 136} {:db/id 212}],
  :block/title "AI",
$ 
$ # illef2 graph에 포함된 모든 tag의 id와 title출력
$ npx @logseq/cli query illef2 '[:find (pull ?b [:db/id :block/title]) :where [?tag :db/ident :logseq.class/Tag] [?b :block/tags ?tag]]'
[{:block/title "Math", :db/id 146}
 {:block/title "note", :db/id 8219}
```

## Task1

- Page node title을 가져와 리스트로 표시한다
- 해당 페이지를 선택해도 아무 동작도 하지 않게 한다

## Task2
- 선택된 Page를 open한다
 - run `open logseq://graph/illef2?page=<uuid>`


## References
- https://github.com/logseq/logseq/tree/master/deps/cli
