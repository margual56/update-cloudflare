# Cloudflare updater
The purpose of this program is to automatically update the IP of a list of Cloudflare records to your current public IP

## Usage
```
Usage: update-cloudflare [OPTIONS] --email <EMAIL> --key <KEY>

Options:
  -l, --list               Lists all records associated with you account and exits
  -r, --records <RECORDS>  List of records (subdomains) to update with your current public IP
  -e, --email <EMAIL>      Email for account admin [env: CLOUDFLARE_EMAIL=]
  -k, --key <KEY>          Global API key [env: CLOUDFLARE_KEY=]
  -h, --help               Print help
  -V, --version            Print version
```

You can export (`export CLOUDFLARE_EMAIL=<your_email>`) the email and key so that you don't have to input them every time.

Remember you can use [crontab.guru](https://crontab.guru/#0_0_*/2_*_*) to assist you in scheduling the run
