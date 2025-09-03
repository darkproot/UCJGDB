# Documentation de L'API

Cette API est le backend qui permet la gestion de la base de donnee de l'`UCJG de Tamdja`.

## Endpoints

---
`Admin`
---


### /get_admin (Methode: POST)

Il permet d'avoir les informations d'un admin

**Envoi**:
```
/get_admin/<int:admin_id>/<int:id>
```

**Retour**:
- `Valide` 
```JSON
{
    "id": 2,
    "password": "qwerty"
    "name": "Nom de l'admin"
}
```

- `Unauthorized` s'il manque __admin_id__ ou l'identifiant n'est pas celui du super admin
```JSON
{
    "err": "Unautorized",
    "res": ""
}
```

### /login (Methode: POST)

Il permet de verifier l'identite d'un administrateur.

**Envoi**:
```JSON
{
    "name": "Nom_de_l_admin",
    "password": "Mot_de_passe"
}
```

**Retour**:
- `Valide`
```JSON
{
    "err": "",
    "id": 2,
    "res": "OK",
    "name": "Nom de l'admin"
}
```

- `Mot de passe incorrect`
```JSON
{
    "err": "Mot de pass incorrect",
    "res": ""
}
```

- `Administrateur indisponible`
```JSON
{
    "err": "Admin indisponible",
    "res": ""
}
```

- `Invalidite des donnees`
```JSON
{
    "err": "Bad data",
    "res": ""
}
```

### /admin (Methode: GET)

Il permet au super utilisateur d'obtenir les infos sur les autres admin.
**Envoi**:
```
/admin/<admin:id>
```

**Retour**:
- `Valider` une liste de tous les admin
```JSON
[
    {
        "id": 1,
        "name": "Redjohn",
        "password": "azerty"
    },
    {
        "id": 2,
        "name": "Alice",
        "password": "qwerty"
    }
]
```
- `Manque de donne` Renvoie un statue `414`

### /sign (Methode: POST)

Il permet au super admin d'ajouter un nouvel admin. La cle __name__ pour le nom, __password__ pour le mot de passe du nouvel admin et ___admin_id__ pour l'identifiant du super admin.
**Envoi**:
```JSON
{
    "name": "Elie",
    "password": "qwerty",
    "admin_id": 1
}
```

**Retour**:
- `Valide` Renvoie le JSON envoyer

- `Unauthorized` s'il manque __admin_id__ ou l'identifiant n'est pas celui du super admin
```JSON
{
    "err": "Unautorized",
    "res": ""
}
```

- `Invalidite des donnees`
```JSON
{
    "err": "Bad data",
    "res": ""
}
```

- `Nom d'admin deja pris`
```JSON
{
    "err": "Nom indisponible",
    "res": ""
}
```

### /del_admin (Methode: POST)

Permet au super admin de supprimer un admin.
**Envoi**:
```JSON
{
    "admin_id": 1,
    "id": 3
}
```

**Retour**:
- `valide` renvoie l'identifiant de l'admin supprimer

- `Manque de l'identifiant du super admin`
```JSON
{
    "err": "Unauthorized",
    "res": ""
}
```

- `Manque de l'identifiant de l'admin a supprimer`
```JSON
{
    "err": "Missing id",
    "res": ""
}
```

- `Identifiant indisponible`
```JSON
{
    "err": "No such admin",
    "res": ""
}
```
---
`Member`
---

### /member (Methode: POST)

Permet d'avoir les informations d'un jeune a parti de son identofiant.
**Envoi**: L'identifiant d'un admin et l'identifiant du jeune
```JSON
{
    "admin_id": 2,
    "id": 2
}
```

**Retour**:
- `Valide`
```JSON
[
    {
        "admin_id": 1,
        "birth_date": "18-02-2009",
        "birth_place": "Douala",
        "classe": "TD",
        "email": "/",
        "id": 2,
        "name": "elbaa woumo",
        "numero": "665 695 804",
        "parent_name": "woumo",
        "parent_numero": "677 786 732",
        "parent_surname": "JP",
        "sexe": "F",
        "surname": "carine"
    }
]
```

- `Manque de l'identifiant du super admin`
```JSON
{
    "err": "Unauthorized",
    "res": ""
}
```

- `Manque de l'identifiant du super admin`
```JSON
{
    "err": "Unauthorized",
    "res": ""
}
```

### /members (Methode: POST)

Permet d'avoir la liste de tous les jeunes et leurs informations'.
**Envoi**: L'identifiant d'un admin
```JSON
{
    "admin_id": 2
}
```

**Retour**:
- `Identifiant d'admin invalide`
```JSON
{
    "err": "Unautorized",
    "res": ""
}
```

- `Valide`
```JSON
[
    {
        "admin_id": 2,
        "birth_date": "08-02-2011",
        "birth_place": "Ndop",
        "classe": "3eme",
        "email": "/",
        "id": 1,
        "name": "chaltouang woumo",
        "numero": "635 645 804",
        "parent_name": "woumo",
        "parent_numero": "677 786 732",
        "parent_surname": "JP",
        "sexe": "M",
        "surname": "manasse"
    },
    {
        "admin_id": 1,
        "birth_date": "18-02-2009",
        "birth_place": "Douala",
        "classe": "TD",
        "email": "/",
        "id": 2,
        "name": "elbaa woumo",
        "numero": "665 695 804",
        "parent_name": "woumo",
        "parent_numero": "677 786 732",
        "parent_surname": "JP",
        "sexe": "F",
        "surname": "carine"
    }
]
```

### /new_member (Methode: POST)

Pour Ajouter un nouveau jeune
**Envoi**: Tous les info indispensable du jeune.
```JSON
{
    "admin_id": 1,
    "birth_date": "18-02-2009",
    "birth_place": "Douala",
    "classe": "TD",
    "name": "woumo",
    "name_p": "woumo",
    "numero": "665 695 804",
    "numero_p": "677 786 732",
    "sexe": "F",
    "surname": "carine",
    "surname_p": "JP"
}
```

**Retour**:
- `Valide`: Les donnees envoyer + l'identifiant

- `Donne insuffisant`:
```JSON
{
    "err": "Bad data", 
    "res": ""
}
```

- `Acces refuse`
```JSON
{
    "err": "Unautorized",
    "res": ""
}
```

### /del_member (Methode: POST)

Pour supprimer un jeune.
**Envoi**: 
```JSON
{
    "admin_id": 2,
    "id": 3
}
```

**Retour**:
- `Valide`: Identifiant du Jeune

- `Acces refuse`
```JSON
{
    "err": "Unautorized",
    "res": ""
}
```

- `Manque de l'identifiant du jeune`
```JSON
{
    "err": "Missing id", 
    "res": ""
}
```

### /update_member (Methode: PUT)
Pour modifier les informations d'un jeune.
Dans la clef `social` si un dictionnaire a la cle "id" le reseau social sera modifier sinon ajouter
**Envoi**:
```JSON
{
    "admin_id": 2,
    "birth_date": "2011-02-08",
    "birth_place": "Ndop",
    "classe": "3eme",
    "email": "/",
    "id": 1,
    "name": "tchaltouang woumo",
    "numero": "635 645 804",
    "parent_name": "woumo",
    "parent_numero": "677 786 732",
    "parent_surname": "JP",
    "sexe": "M",
    "surname": "manasse",
    "social": [
        {"name": "facebook", "pseudo": "redjohn"},
        {"id": 3, "name": "snapchat", "pseudo": "alice"}
    ]
}
```

- `Acces refuse`
```JSON
{
    "err": "Unautorized",
    "res": ""
}
```

- `Manque de l'identifiant du jeune`
```JSON
{
    "err": "Missing id", 
    "res": ""
}
```

- `Donne insuffisant`:
```JSON
{
    "err": "Bad data", 
    "res": ""
}
```
