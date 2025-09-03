from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class Admin(db.Model):
    __tablename__ = 'admin'
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(50), unique=True, nullable=False)
    password = db.Column(db.String(50), nullable=False)

    def __repr__(self):
        return f"Admin({self.name}, {self.password})"
    
class SocialMedia(db.Model):
    __tablename__ = "social_media"
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(50), nullable=False)
    pseudo = db.Column(db.String(50), nullable=False)
    member_id = db.Column(db.Integer, db.ForeignKey("member.id"), nullable=False)
    
    def __repr__(self):
        return f"SocialMedia({self.name}: {self.pseudo})"
    
class Member(db.Model):
    __tablename__ = "member"
    id = db.Column(db.Integer, primary_key=True)
    admin_id = db.Column(db.Integer, db.ForeignKey("admin.id"), nullable=False)
    name = db.Column(db.String(100), nullable=False)
    surname = db.Column(db.String(50), nullable=False)
    sexe = db.Column(db.String(2), nullable=False)
    birth_date = db.Column(db.String(11), nullable=False)
    birth_place = db.Column(db.String(100), nullable=False)
    classe = db.Column(db.String(50))
    numero = db.Column(db.String(50), nullable=False)
    parent_name = db.Column(db.String(50))
    parent_surname = db.Column(db.String(50))
    parent_numero = db.Column(db.String(50), nullable=False)
    email = db.Column(db.String(50))

    def __repr__(self):
        return f"JEUNE:\n\tNom: {self.name}\n\tPrenom: {self.surname}\n\tSexe: {self.sexe}"
