db.createUser({
    user: 'admin',
    pwd: 'admin',
    roles: [
        {
            role: 'readWrite',
            db: 'order-history',
        }
    ],
});

db = new Mongo().getDB("order-history");
db.createCollection('order-history');
