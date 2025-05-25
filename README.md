# image-upload-webpage

> [!NOTE]
> THE LINODE IS POWERED OFF

## How To set up next time

1. Create a virtual environment (I used [Linode](https://cloud.linode.com/))
1. Buy a domain name `my-domain-example.com`
1. Make the DNS point to the IP of the virtual environment
   1. This can be done via Linode or via the Domain Registrar
   1. (I used the domain registrar, i.e., [loopia](https://loopia.se/))
1. Pull this repository to the virtual environment
1. Create a `.env` file in the root directory of the project with the following content:

   ```env
   DOMAIN=my-domain-example.com
   LE_EMAIL=my-email@example.com
   ```

1. Create a `.env` file in the `./server` directory with the following content:

   ```env
    JWT_SECRET='<your-secret-key>'
    JWT_EXPIRATION_TIME='3600'
    SHARED_PASSWORD_HASH='<the-hashed-password>'
    SHARED_PASSWORD='<the-real-password>'
    PORT='3000'
   ```

1. If you want to issue a TLS certificate,
   you can use [Let's Encrypt](https://letsencrypt.org/) with [Certbot](https://certbot.eff.org/).

   1. Comment out the `HTTPS` server block in `./nginx/templates/site.conf.template`.
   1. Run your Nginx proxy and your webapp as:

      ```bash
      docker compose up -d nginx webapp
      ```

   1. Execute the provisioning command:

      ```bash
      make init-cert
      ```

   1. Uncomment the `HTTPS` server block in `./nginx/templates/site.conf.template`.
   1. Follow the remaining instructions (since you've now issued a TLS certificate for your webapp).

1. Build the docker image using docker compose:

   ```bash
   make build up
   ```

1. Access the webpage via `https://my-domain-example.com`

## ToDo - General

1. ~~Input sanitization~~
2. ~~Authentication and Authorization~~
3. ~~Rate limiting~~
4. ~~Background processing of thumbnails~~ -- Solved by compiling with a higher optimization level
5. ~~Don't retrieve all thumbnails on upload (filter)~~
6. ~~Introduce TLS~~
7. Double check the JWT expiration time
8. Caching
9. Indicator that images are being processed
10. Video support
11. Make the _gallery_-scroll on phone swipeable, not just clickable.
12. Prevent gallery images from being movable.
13. Lazily load high-quality images in gallery
14. Make sure that upload button works.
15. Improve tab name (currently "Vite + Vue + TS")
