from typing import List, Dict
import sqlite3

######################################## Admin ###############################

def get_db_connection(): 
    conn = sqlite3.connect('./instance/database.db')
    conn.row_factory = sqlite3.Row
    return conn

def get_admins() -> List[Dict[str, str]]:
    """Pour avoir tous les administrateur"""
    conn = get_db_connection()
    cursor = conn.execute('SELECT * FROM admin')
    rows = cursor.fetchall()
    conn.close()

    output = [dict(row) for row in rows]
    return output

def get_admin_info(id: int) -> Dict[str, str]:
    """Pour avoir les info d'un admin"""
    conn = get_db_connection()
    cursor = conn.execute('SELECT * FROM admin WHERE id == ?', (id,))
    row = cursor.fetchall()
    conn.close()

    output = [dict(r) for r in row]
    return output


def remove_admin(id: int):
    """Pour supprimer un administrateur"""
    conn = get_db_connection()
    conn.execute("DELETE FROM admin WHERE id = ?", (id,))
    conn.commit()
    conn.close()

######################################## Member ###############################

def remove_member(id: int): 
    """Pour supprimer un Jeune"""
    conn = get_db_connection()
    conn.execute("DELETE FROM member WHERE id = ?", (id,))
    conn.commit()
    conn.close()

def get_members() -> List[Dict[str, str]]:
    """Pour avoir tous les Jeunes"""
    conn = get_db_connection()
    cursor = conn.execute('SELECT * FROM member')
    rows = cursor.fetchall()
    conn.close()

    output = [dict(row) for row in rows]
    return output

def get_member(id: int) -> List[Dict[str, str]]:
    """Avoir les informations d'un Jeune"""
    conn = get_db_connection()
    cursor = conn.execute('SELECT * FROM member WHERE id = ?', (id,))
    row = cursor.fetchall()
    conn.close()

    return [dict(r) for r in row]

def remove_member(id: int):
    """Pour supprimer un Jeune"""
    conn = get_db_connection()
    conn.execute("DELETE FROM member WHERE id = ?", (id,))
    conn.commit()
    conn.close()

######################################## Social Media ###############################

def get_social_media(id: int) -> List[Dict[str, str]]:
    """Pour avoir tous les reseaus sociaux d'un jeune"""
    conn = get_db_connection()
    cursor = conn.execute('SELECT * FROM social_media WHERE id = ?', (id,))
    rows = cursor.fetchall()
    conn.close()

    output = [dict(row) for row in rows]
    return output
