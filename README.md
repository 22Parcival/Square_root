# Square_root
## Square_root le nouveau format de fichier en .toor
* Square_root est un nouveau format de fichier crée à l'occasion d'un hackathon sur le serveur Discord du youtubeur Aywen, il à été fait avec amour et le temps disponible que j'avais
### Ce projet comporte deux parties :
* **L'encodeur fait en Rust** il prend l'image qu'on lui donne, la redimensionne puis génère un fichier en binaire `.toor` à partir des pixels extraits de l'image.
* **Le decodeur web** il lit le fichier `.toor` puis affiche l'image avec des Root à la place des pixels

## Les trucs pratiques si vous voulez que mon code marche :
* **Rust et cargo** (pratique pour un projet écrit en rust) [lien pour installer Rust](https://www.rust-lang.org/tools/install)

* **Un navigateur web** (si vous en n'avez pas je ne sais pas comment vous faites pour vivre)
---
## tuto pour utiliser l'encodeur
* Prenez votre image que vous voulez rootisez puis placez la dans le projet (vous pouvez crée un dossier dans le projet pour placez vos images).
* Ouvrez le terminal à la racine du projet puis exécutez la commande :
```cargo run -- --input chemin/encorechemin/votre_image.jpg --output chemin/encorechemin/votre_image.toor```

par exemple moi je fait : ```cargo run -- --input test_img/aywen1.jpg --output test_img/test1.toor```

 ---
 ## Tuto pour utiliser le décodeur (pour pouvoir lire votre fichier `.toor`)
 * ouvrez le dossier `web_lecteur` puis lancer le fichier index.html (avec live server ou juste en ouvrant le fichier dans votre navigateur web)

 * une fois le site ouvert cliquez sur le bouton `Parcourir`, sélectionner votre fichier en `.toor` l'image devrais ensuite ce lancez après une ou deux seconde si ce n'est pas instantané