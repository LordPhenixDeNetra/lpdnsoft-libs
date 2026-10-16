hello = Bonjour
servers-count = { $count ->
    [one] { $count } serveur
   *[other] { $count } serveurs
}
