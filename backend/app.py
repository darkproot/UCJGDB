from flask import Flask, jsonify, request
from models import db, Admin, Member, SocialMedia
from config import Config
from flask_cors import CORS
from database_handler import get_admins, remove_admin, get_members, get_admin_info, remove_member, get_member, get_social_media

app = Flask(__name__)
app.config.from_object(Config)
cors = CORS(app, origins='*')
db.init_app(app)
SUPER_ADMIN = 1

######################################## Admin ###############################

# Avoir les informations d'un admin
@app.route('/get_admin/<int:admin_id>/<int:id>', methods=['GET'])
def getAdminInfo(admin_id: int, id: int):
	if not admin_id or int(admin_id ) != SUPER_ADMIN: return jsonify({"err": "Unautorized", "res": ""})
	if not id: return jsonify({"err": "Missing id", "res": ""})
	admin = Admin.query.get_or_404(id)
	result = {"id": admin.id, "name": admin.name, "password": admin.password}

	return jsonify(result)

# Avoir tous les Admin
@app.route("/admin/<int:admin_id>", methods=['GET'])
def get_admin(admin_id: int):
    if admin_id: 
        if int(admin_id )== SUPER_ADMIN: return jsonify(get_admins())
    return jsonify({"err": "Unautorized", "res": ""})


# Ajouter un Admin
@app.route("/sign", methods=["POST"])
def sign():
    data = request.get_json()
    if not data.get('admin_id'): return jsonify({"err": "Unautorized", "res": ""})
    if data.get('admin_id') != 1: return jsonify({"err": "Unautorized", "res": ""})
    else:
        if not data.get('name') or not data.get('password'): return jsonify({"err": "Bad data", "res": ""})
        else:
            admins = get_admins()
            admins_names = [admin.get('name') for admin in admins]
            if data.get('name') in admins_names: return jsonify({"err": "Nom indisponible", "res": ""})
            else: 
                admin = Admin(
                	name=data.get('name'), 
                	password=data.get('password')
                )
                db.session.add(admin)
                db.session.commit()
                return data, 200


# Verifier un Admin
@app.route("/login", methods=['POST'])
def login():
    data = request.get_json()
    if not data.get('name') or not data.get('password'): return jsonify({"err": "Bad data", "res": ""})
    else:
        admins = get_admins()
        for admin in admins:
            if data.get('name') == admin['name']:
                result = {"err": "", "res": ""}
                if data.get('password') != admin['password']: 
                    result['err'] = "Mot de pass incorrect"
                    return jsonify(result)
                else:
                    result['res'] = "OK"
                    result['id'] = admin['id']
                    result['name'] = admin['name']
                    return jsonify(result)
        return jsonify({"err": "Admin indisponible", "res": ""})


# Supprimer un Admin
@app.route("/del_admin", methods=['POST'])
def delete_admin():
    data = request.get_json()
    if data.get('admin_id') != 1: return jsonify({"err": "Unauthorized", "res": ""})
    elif not data.get('id'): return jsonify({"err": "Missing id", "res": ""})
    else: 
        admins = get_admins()
        for admin in admins:
            if admin.get('id') == data.get('id'):
                remove_admin(data.get('id'))
                return jsonify(data.get("id"))
        return jsonify({"err": "No such admin", "res": ""})
    
    
######################################## Member ###############################    

# Avoir tous les Jeunes
@app.route('/members', methods=['POST'])
def getMembers():
    data = request.get_json()
    if data.get('admin_id'): 
        if len(get_admin_info(data.get('admin_id'))): return jsonify(get_members())
    return jsonify({"err": "Unautorized", "res": ""})

# Avoir un jeune
@app.route('/member', methods=['POST'])
def getMember():
	data = request.get_json()
	if not data.get('admin_id'): return jsonify({"err": "Unautorized", "res": ""})
	if not len(get_admin_info(data.get('admin_id'))): return jsonify({"err": "Unautorized", "res": ""})
	elif not data.get('id'): return jsonify({"err": "Missing id", "res": ""})
	else:
		res = get_member(data.get('id'))
		var = []
		if res[0]: 
			socials = SocialMedia.query.all()
			for social in socials:
				if social.member_id == data.get('id'):
					var.append({"member_id": social.member_id, "pseudo": social.pseudo, "name": social.name, "id": social.id})
		res[0]['social'] = var
		return jsonify(res)

# Pour modifier les informations d'un jeune
@app.route('/update_member', methods=['PUT'])
def updateMember():
	data = request.get_json()
	if not data.get('admin_id'): return jsonify({"err": "Unautorized", "res": ""})
	if not len(get_admin_info(data.get('admin_id'))): return jsonify({"err": "Unautorized", "res": ""})
	elif not data.get('id'): return jsonify({"err": "Missing id", "res": ""})
	elif not get_member(data.get('id')): return jsonify({"err": "unknowned Member", "res": ""})
	else:
		# Verifier la validiter de tous les elements
		cache = not data.get('name') or not data.get('surname') or not data.get('sexe') or not data.get('birth_date') or not data.get('birth_place') or not \
			data.get('numero') or not data.get('parent_numero')

		if cache: return jsonify({"err": "Bad data", "res": ""})
		else:
			member = Member(
					id=data.get('id'),
					admin_id=data.get('admin_id'),
					name=data.get('name'),
					surname=data.get('surname'),
					sexe=data.get('sexe'),
					birth_date=data.get('birth_date'),
					birth_place=data.get('birth_place'),
					classe=data.get('classe') or '/',
					numero=data.get('numero'),
					parent_name=data.get('parent_name') or '/',
					parent_surname=data.get('parent_surname') or '/',
					parent_numero=data.get('parent_numero'),
					email=data.get('email') or '/'
				)

			if data.get('social'):
				socials: list[dict[str, str]] = data.get('social')
				for social in socials:
					if social.get('id'):
						var = SocialMedia(
							id=social.get('id'),
							name=social.get('name'),
							pseudo=social.get('pseudo'),
							member_id=data.get('id')
						) 
						db.session.merge(var)
					else:
						var = SocialMedia(
								name=social.get('name'),
								pseudo=social.get('pseudo'),
								member_id=data.get('id')
							)
						db.session.add(var)
					db.session.commit()

			db.session.merge(member)
			db.session.commit()
			data['id'] = member.id
			return data, 200

# @app.route('/get', methods=['GET'])
# def get():
# 	social = SocialMedia.query.all()
# 	result = []
# 	for s in social: result.append({"member_id": s.member_id, "pseudo": s.pseudo, "name": s.name, "id": s.id})
# 	return jsonify(result)

# @app.route('/delete/<int:id>', methods=['DELETE'])
# def fel(id: int):
# 	social = SocialMedia.query.get_or_404(id)
# 	db.session.delete(social)
# 	db.session.commit()
# 	return jsonify({"data": social.member_id})

# Ajouter un jeune
@app.route("/new_member", methods=["POST"])
def add_member():
	data = request.get_json()
	if not data.get('admin_id'): return jsonify({"err": "Unautorized", "res": ""})
	if not len(get_admin_info(data.get('admin_id'))): return jsonify({"err": "Unautorized", "res": ""})
	else:
		# Verifier la validiter de tous les elements
		cache = not data.get('name') or not data.get('surname') or not data.get('sexe') or not data.get('birth_date') or not data.get('birth_place') or not \
			data.get('numero') or not data.get('parent_numero')

		if cache: return jsonify({"err": "Bad data", "res": ""})
		else:
			member = Member(
					admin_id=data.get('admin_id'),
					name=data.get('name'),
					surname=data.get('surname'),
					sexe=data.get('sexe'),
					birth_date=data.get('birth_date'),
					birth_place=data.get('birth_place'),
					classe=data.get('classe') or '/',
					numero=data.get('numero'),
					parent_name=data.get('name_p') or '/',
					parent_surname=data.get('surname_p') or '/',
					parent_numero=data.get('parent_numero'),
					email=data.get('email') or '/'
				)
			db.session.add(member)
			db.session.commit()
			data['id'] = member.id
			return data, 200

# Supprimer un Jeune
@app.route("/del_member", methods=['POST'])
def delete_member():
	data = request.get_json()
	if not data.get('admin_id'): return jsonify({"err": "Unautorized", "res": ""})
	if not len(get_admin_info(data.get('admin_id'))): return jsonify({"err": "Unautorized", "res": ""})
	elif not data.get('id'): return jsonify({"err": "Missing id", "res": ""})
	else: 
		remove_member(data.get('id'))
		return jsonify(data)
			    

if __name__ == "__main__":
	with app.app_context(): 
		db.create_all()
	app.run(debug=True, host='0.0.0.0', port=5000)
