
# 📘 Guide de FireWall

## 🔥 Contexte
Dans les profondeurs d’un ordinateur, des **virus intelligents** s’affrontent afin de prouver leur suprématie.

Chaque bot cherche à devenir le plus robuste, mais surtout, **le dernier encore actif** dans un système en perpétuel déclin. 

Mais tous craignent un ennemi plus impitoyable : Le **Firewall**.

Ce pare-feu brûle et supprime les processus les plus faibles, **réduisant progressivement** la mémoire disponible.

---
## 🤖 Caractéristique du bot
- HP: 100
- Team
- Score
- Position
- Inventaire : aucune limite
- Effets
- Distance de vision : 3 (7x7)
```

⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜🟦🟦🟦⬜⬜
⬜⬜🟦🧍🟦⬜⬜
⬜⬜🟦🟦🟦⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜

```


## ⏳ Déroulement d'une partie
### Début
- En début de partie, les bots sont positionné aléatoirement sur la carte.
- Le FireWall commence à se propager dans un certain pattern

### Résolution d'un tick (0.5ms)
1. Première **action** du bot
2. Affectation des **dégâts** des pièges et firewall
3. Affectation des **dégâts** des projectiles
4. Déplacement des **projectiles**
5. Affectation des **dégâts** des projectiles
6. FireWall se propage
7. Affectation des **dégâts** des nouvelles tuiles du FireWall

### Fin
- S'il ne reste plus qu'un bot sur la carte, il sera couronné vainqueur de la partie.

### Spécificités
- Lorsqu'un bot est éliminé par quelqu'un, son inventaire est transféré dans celui du bot qui l'a éliminé.
- Lorsqu'un bot est éliminé par le FireWall, son inventaire est détruit.
- Un piège fait des dégâts, puis se détruit durant le tick.

### Pointage

| Points     | Action |
|------------|--------|
| +150       | Remporter une partie (être le dernier en vie) |
| (nbDépart - nbRestant) * 2  | Nombre de joueurs déjà éliminés lors de l'élimination |
| +40       | Éliminer un bot adverse |
| +5        | Ouvrir un coffre |
| +2        | Utiliser un buff |
| +2         | Survivre 5 secondes |
| +1        | Détruire un obstacle |
| -10        | Marcher dans un piège |
| -30        | Se faire éliminer par un bot adverse |
| -75       | Forcer un segfault (abandon volontaire) |

## 🎮 Actions du Bot

### 🚶 `move(direction)`
**Description :**  
Se déplace vers une case à une position absolue à une case du bot.

**Paramètre :**
- `direction`: `{ x: number, y: number }`

**Exemple :**
```ts
bot.move({ x: 120, y: 57 });
```

---

### 🌀 `phase(direction)`
**Description :**  
Traverse un ou des obstacles dans une **direction cardinale**.
Il est possible de traverser plus d'un mur par **phase**.

**Paramètre :**
- `direction`: `"up" | "down" | "left" | "right"`

**Exemple :**
```ts
bot.phase("up");
```

---

### 🗝️ `openChest(position)`
**Description :**  
Ouvre un coffre à une position donnée.
Tous les objets sont transférés dans l'inventaire du bot.
Un coffre peut être ouvert une fois par bot par partie.
Le coffre doit être à une case du bot.

**Paramètre :**
- `position`: `{ x: number, y: number }`

**Exemple :**
```ts
bot.openChest({ x: 3, y: 7 });
```

---

### BUFFS : 🧪 `useItemBuff(item)`
**Description :**  
Utilise un buff.

**Exemple :**
```ts
bot.useItemBuff(item);
```

### PROJECTILE : 🏹 `useItemProjectile(item, direction)`
**Description :**  
Utilise le projectile dans une certaine direction.

**Exemple :**
```ts
bot.useItemProjectile(item, "left");
```

### PLACED : 🏔️ `useItemPlaced(item, position, rotation)`
**Description :**  
Place un objet sur la carte selon une position et un pattern.

**Exemple :**
```ts
bot.useItemPlaced(item, { x: 1, y: 0 }, rotation);
```

### ☢️ `useItemNuke()`
**Description :**  
Déclenche une attaque globale qui élimine tous les joueurs sur la carte.

**Exemple :**
```ts
bot.useItemNuke(item);
```

---

### 💀 `segFault()`
**Description :**  
Abandonne la partie (suicide).

**Exemple :**
```ts
bot.segFault();
```

---

### ⏭️ `doNothing()`
**Description :**  
Passe une action sans rien faire.

**Exemple :**
```ts
bot.doNothing();
```

---
## 🗺️ Carte & Terrain

### 🏝️ Type de terrain

**Exemple :**
```ts
console.log(gameState.ground.data) // Donne le type de terrain dans le champs de vision du bot
```
---
### 🟩 pcb
**Description :** PCB - Circuit imprimé
#### C'est là où le bot peut se déplacer.

---

### 🟥 Firewall
**Description :** FireWall - Pare-feu
#### C'est la zone qui se réduit au fur et à mesure de la partie. Elle enlève 10hp/tick.
Patterns :
- 4 coins de la carte
- Centre de la carte
- 1 coin de la carte

---

### ⬛️ Via
**Description :** VIA - Trou dans le circuit imprimé
#### C'est un trou dans la plaquette où les bots ne peuvent ni se déplacer, ni passer au travers.

---

### Objects
**Description :**  
- 🟨 Résistance (Mur)
- 🟫 Coffres
- 🟣 Projectiles
- 🔵 Bots

---

### 📍 `getCell(relativePosition)`
**Description :**  
Retourne le type de case à une position **relative** au joueur.

**Paramètre :**
- `{ x: number, y: number }`

---

### 🌐 `getGlobalCell(position)`
**Description :**  
Retourne le type de case à une position **absolue** sur la carte.

**Exemple :**
```ts
const cell = bot.getGlobalCell({ x: 12, y: 3 });
```

---

### 🖨️ Affichage

### 🧾 `print()`
**Description :**  
Affiche dans la console :
- Position du joueur
- Nombre de kills
- Carte visuelle avec la position `◉`

**Symboles :**

| Symbole | Signification         |
|---------|------------------------|
| ◼️  | Vide                    |
| 🟩   | Circuit imprimé (pcb) |
| 🟥 | FireWall              |
| ◼️  | Via                   |
| `▩` | Coffre |
| 🟨  | Résistance            |
| 🔵       | Joueur                |

---

## Objets

### 🧪 BUFFS
*Donne 10hp par utilisation*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Repair"` |
| **Cooldown** | 2           |
| **Quantité** | 5           |
| **Effect**   | Heal        |
| **Power**    | 10          |
| **Duration** | 0           |
---
*Donne 100hp*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"FullRepair"` |
| **Cooldown** | 10           |
| **Quantité** | 1           |
| **Effect**   | Heal        |
| **Power**    | 100          |
| **Duration** | 0           |
---
*Donne 10shield par utilisation*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Buffer"` |
| **Cooldown** | 2           |
| **Quantité** | 5           |
| **Effect**   | Shield        |
| **Power**    | 10          |
| **Duration** | 0           |
---
*Donne 100shield*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"FullBuffer"` |
| **Cooldown** | 10           |
| **Quantité** | 1           |
| **Effect**   | Shield        |
| **Power**    | 100          |
| **Duration** | 0           |
---
*Donne 100hp et 100shield*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"RepairAndBuffer"` |
| **Cooldown** | 10           |
| **Quantité** | 1           |
| **Effect**   | HealAndShield        |
| **Power**    | 100          |
| **Duration** | 0           |
---
*Enlève tous les cooldowns actifs.*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Overclock"` |
| **Cooldown** | 0           |
| **Quantité** | 2           |
| **Effect**   | Haste        |
| **Power**    | 10          |
| **Duration** | 0           |
---
*Boost le damage x2 durant 30s*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"CryptoMiner"` |
| **Cooldown** | 30           |
| **Quantité** | 2           |
| **Effect**   | Score        |
| **Power**    | 10          |
| **Duration** | 30           |
---
*Augmente le score de 10pts par utilisation*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Sudo"` |
| **Cooldown** | 0           |
| **Quantité** | 5           |
| **Effect**   | Damage        |
| **Power**    | 10          |
| **Duration** | 0           |
---

### 🏹 PROJECTILES
*Attaque de mélée (toujours dans l'inventaire)*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Delete"` |
| **Cooldown** | 6           |
| **Quantité** | Infinite           |
| **TTL**   | 1        |
| **Damage**    | 50          |
| **Pattern** | `"Single"`           |
---
*Pistolet*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"ByteCannon"` |
| **Cooldown** | 1           |
| **Quantité** | 12           |
| **TTL**   | 3        |
| **Damage**    | 10          |
| **Pattern** | `"Single"`           |
---
*Sniper*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Ping"` |
| **Cooldown** | 5           |
| **Quantité** | 5           |
| **TTL**   | 15        |
| **Damage**    | 20          |
| **Pattern** | `"Single"`           |
---
*Minigun*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"DDOS"` |
| **Cooldown** | 0           |
| **Quantité** | 25           |
| **TTL**   | 8        |
| **Damage**    | 5          |
| **Pattern** | `"Single"`           |
---
*Shotgun*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Multicast"` |
| **Cooldown** | 3           |
| **Quantité** | 2           |
| **TTL**   | 2        |
| **Damage**    | 20          |
| **Pattern** | `"Line"`           |
---
*Tire en croix*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Cursor"` |
| **Cooldown** | 4           |
| **Quantité** | 2           |
| **TTL**   | 5        |
| **Damage**    | 15          |
| **Pattern** | `"Star"`           |
---
*Hammer*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Broadcast"` |
| **Cooldown** | 3           |
| **Quantité** | 3           |
| **TTL**   | 1        |
| **Damage**    | 20          |
| **Pattern** | `"Box"`           |
---
*Hammer Projectiles*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Flood"` |
| **Cooldown** | 5           |
| **Quantité** | 1           |
| **TTL**   | 3        |
| **Damage**    | 15          |
| **Pattern** | `"Box"`           |
---

### ☢️ NUKE
*Nuke : élimine tous sauf le bot qui a activé et les bots avec 100hp et 100shield*
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"Bluescreen"` |
| **Cooldown** | 10           |
| **Quantité** | 1           |
| **Damage**    | 199          |
---

### 🏔️ PLACED
#### Les murs ne peuvent pas être superposés, alors évitez de les spammer sinon vous les perdrez !

| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"SimpleResistance"` |
| **Cooldown** | 0           |
| **Quantité** | 4           |
| **Portée**   | 4           |
| **Type**     | Wall        |
| **Pattern**  | `"Single"`       |
---
*Mur 3x1*
| Attribut     | Valeur |
| ------------ | ------ |
| **Nom**      | `"Resistance"`   |
| **Cooldown** | 2      |
| **Quantité** | 2      |
| **Portée**   | 2      |
| **Type**     | Wall   |
| **Pattern**  | `"Rectangle"`   |
---
*Mur 5x2*
| Attribut     | Valeur             |
| ------------ | ------------------ |
| **Nom**      | `"HugeResistance"`          |
| **Cooldown** | 4                  |
| **Quantité** | 1                  |
| **Portée**   | 2                  |
| **Type**     | Wall               |
| **Pattern**  | `"Rectangle"` |
---
*Mur autour du bot*
| Attribut     | Valeur             |
| ------------ | ------------------ |
| **Nom**      | `"DefensiveResistance"`          |
| **Cooldown** | 4                  |
| **Quantité** | 1                  |
| **Portée**   | 2                  |
| **Type**     | Wall               |
| **Pattern**  | `"Box"` |
---
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"WindowsDefender"` |
| **Cooldown** | 1           |
| **Quantité** | 4           |
| **Portée**   | 2           |
| **Type**     | Trap        |
| **Pattern**  | `"Single"`       |
| **Damage**   | 10        |
---
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"CrowdStrike"` |
| **Cooldown** | 2           |
| **Quantité** | 4           |
| **Portée**   | 5           |
| **Type**     | Trap        |
| **Pattern**  | `"Single"`       |
| **Damage**   | 5        |
---
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"FactoryReset"` |
| **Cooldown** | 10           |
| **Quantité** | 1           |
| **Portée**   | 0           |
| **Type**     | Trap        |
| **Pattern**  | `"Single"`       |
| **Damage**   | 40        |
---
| Attribut     | Valeur      |
| ------------ | ----------- |
| **Nom**      | `"McAfee"` |
| **Cooldown** | 0           |
| **Quantité** | 1           |
| **Portée**   | 0           |
| **Type**     | Trap        |
| **Pattern**  | `"Single"`       |
| **Damage**   | 1        |
---

## Patterns
- Selon directions cardinales (up|down|left|right)
- Selon la portée

**Les exemples sont au Nord et à une portée de 1**
### Single
```

⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜🟦⬜⬜⬜
⬜⬜⬜🧍⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜

```
---
### Box/Square
```

⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜🟦🟦🟦⬜⬜
⬜⬜🟦🧍🟦⬜⬜
⬜⬜🟦🟦🟦⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜

```
---
### Star
```

⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜🟦⬜⬜⬜
⬜⬜🟦🧍🟦⬜⬜
⬜⬜⬜🟦⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜

```
---
### Rectangle
```

⬜⬜⬜⬜⬜⬜⬜
⬜🟦🟦🟦🟦🟦⬜
⬜🟦🟦🟦🟦🟦⬜
⬜⬜⬜🧍⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜

```
---
