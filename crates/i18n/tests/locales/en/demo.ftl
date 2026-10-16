hello = Hello
servers-count = { $count ->
    [one] { $count } server
   *[other] { $count } servers
}
only-en = Only English
