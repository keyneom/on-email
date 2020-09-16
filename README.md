# on-email

Email is actually pretty awesome! Use it as an async durable work queue and backend for whatever you want!

I'm planning on adding support for config parameters passed by command line (email address, password, etc.) to instead be passed by a file but only once clap supports it.

Set this to run on a schedule and execute whatever logic you'd like with matching emails. You can tag emails matching certain parameters and they get output as JSON through stdout. You can pass these emails into another program to do any kind of processing you'd like.

I plan on adding comment functionality to my static website that will fill out an email template after you click to comment. Send the email and this will watch for emails matching the defined criteria. I then pass the matching emails over to another program that will add the comment to my static website and push out a new update.

The fun part is that email actually is a great match for comments in a lot of ways. You have spam filters built in and they can be customized. You don't need to worry about getting DDOSed because rate-limiting and all kinds of other stuff is pretty much out of the box with your email address.

I'll add more documentation over time but it attempts to connect over encrypted imap. Other stuff could be supported over time.

Here's an example of a command and the output:
```
> on-email --domain String --email String --password String --tag bill --subject "Credit Card Payment Due" watch --tag new-comment --subject "Comment for:"
{ "subject": "Credit Card Payment Due", . . . "tag": "bill" }
{ "subject": "Comment for: keyneom.github.io", . . . "tag": "new-comment" }
```
