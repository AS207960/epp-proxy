use chrono::prelude::*;
use futures::SinkExt;
use tokio::io::AsyncBufReadExt;

fn decode_line(data: &str, host: &str) -> Result<super::proto::DACResponse, ()> {
    let mut parts = data.split(',').collect::<Vec<_>>();
    if parts.len() < 2 {
        error!("Invalid syntax from {}: expected at least two parts", host);
        Err(())
    } else {
        parts.reverse();
        let domain = parts.pop().unwrap();
        let registered = parts.pop().unwrap();

        match registered {
            "C" => {
                if parts.len() != 4 {
                    error!(
                        "Invalid syntax from {}: expected at 6 parts for a usage response",
                        host
                    );
                    Err(())
                } else {
                    let limit_60_const = parts.pop().unwrap();
                    let limit_60 = parts.pop().unwrap();
                    let limit_24_const = parts.pop().unwrap();
                    let limit_24 = parts.pop().unwrap();

                    if limit_60_const != "60" || limit_24_const != "86400" {
                        error!("Invalid syntax from {}: invalid usage response", host);
                        Err(())
                    } else {
                        let limit_60 = match limit_60.parse::<u64>() {
                            Ok(l) => l,
                            Err(_) => {
                                error!("Invalid syntax from {}: invalid int", host);
                                return Err(());
                            }
                        };
                        let limit_24 = match limit_24.parse::<u64>() {
                            Ok(l) => l,
                            Err(_) => {
                                error!("Invalid syntax from {}: invalid int", host);
                                return Err(());
                            }
                        };

                        let usage = super::proto::Usage {
                            usage_60: limit_60,
                            usage_24: limit_24,
                        };

                        match domain {
                            "#usage" => Ok(super::proto::DACResponse::Usage(usage)),
                            "#limits" => Ok(super::proto::DACResponse::Limits(usage)),
                            o => {
                                error!("Invalid syntax from {}: invalid domain for a usage response ({})", host, o);
                                Err(())
                            }
                        }
                    }
                }
            }
            "B" => {
                if parts.len() != 1 {
                    error!(
                        "Invalid syntax from {}: expected 3 parts for an AUB response",
                        host
                    );
                    Err(())
                } else {
                    let delay = parts.pop().unwrap();
                    let delay = match delay.parse::<u64>() {
                        Ok(l) => l,
                        Err(_) => {
                            error!("Invalid syntax from {}: invalid int", host);
                            return Err(());
                        }
                    };
                    Ok(super::proto::DACResponse::Aub(super::proto::Aub {
                        domain: domain.to_string(),
                        delay,
                    }))
                }
            }
            "I" => {
                error!("Invalid syntax error received from {}", host);
                Err(())
            }
            r => {
                match parts.len() {
                    0 => {
                        let registered = match r {
                            "Y" => true,
                            "N" => false,
                            _ => {
                                error!("Invalid syntax from {}: invalid registered status", host);
                                return Err(());
                            }
                        };

                        Ok(super::proto::DACResponse::DomainRT(
                            super::proto::DomainRT {
                                domain: domain.to_string(),
                                registered,
                                detagged: false,
                                created: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                                expiry: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                                tag: String::default(),
                            },
                        ))
                    }
                    4 => {
                        let detagged = parts.pop().unwrap();
                        let created = parts.pop().unwrap();
                        let expiry = parts.pop().unwrap();
                        let tag = parts.pop().unwrap();

                        let registered = match r {
                            "Y" => true,
                            "N" => false,
                            _ => {
                                error!("Invalid syntax from {}: invalid registered status", host);
                                return Err(());
                            }
                        };
                        let detagged = match detagged {
                            "Y" => true,
                            "N" => false,
                            _ => {
                                error!("Invalid syntax from {}: invalid bool", host);
                                return Err(());
                            }
                        };
                        let created = match chrono::NaiveDate::parse_from_str(created, "%Y-%m-%d") {
                            Ok(d) => d,
                            Err(e) => {
                                error!("Invalid syntax from {}: invalid date {}", host, e);
                                return Err(());
                            }
                        };
                        let expiry = match chrono::NaiveDate::parse_from_str(expiry, "%Y-%m-%d") {
                            Ok(d) => d,
                            Err(e) => {
                                error!("Invalid syntax from {}: invalid date {}", host, e);
                                return Err(());
                            }
                        };

                        Ok(super::proto::DACResponse::DomainRT(
                            super::proto::DomainRT {
                                domain: domain.to_string(),
                                registered,
                                detagged,
                                created,
                                expiry,
                                tag: tag.to_string(),
                            },
                        ))
                    }
                    6 => {
                        let detagged = parts.pop().unwrap();
                        let suspended = parts.pop().unwrap();
                        let created = parts.pop().unwrap();
                        let expiry = parts.pop().unwrap();
                        let status = parts.pop().unwrap();
                        let tag = parts.pop().unwrap();

                        let registered = match r {
                            "Y" => super::proto::DomainRegistered::Registered,
                            "N" => super::proto::DomainRegistered::Available,
                            "E" => super::proto::DomainRegistered::NotWithinRegistry,
                            "R" => super::proto::DomainRegistered::RulesPrevent,
                            _ => {
                                error!("Invalid syntax from {}: invalid registered status", host);
                                return Err(());
                            }
                        };
                        let detagged = match detagged {
                            "Y" => true,
                            "N" => false,
                            _ => {
                                error!("Invalid syntax from {}: invalid bool", host);
                                return Err(());
                            }
                        };
                        let suspended = match suspended {
                            "Y" => true,
                            "N" => false,
                            _ => {
                                error!("Invalid syntax from {}: invalid bool", host);
                                return Err(());
                            }
                        };
                        let created = match chrono::NaiveDate::parse_from_str(created, "%Y-%m-%d") {
                            Ok(d) => d,
                            Err(e) => {
                                error!("Invalid syntax from {}: invalid date {}", host, e);
                                return Err(());
                            }
                        };
                        let expiry = match chrono::NaiveDate::parse_from_str(expiry, "%Y-%m-%d") {
                            Ok(d) => d,
                            Err(e) => {
                                error!("Invalid syntax from {}: invalid date {}", host, e);
                                return Err(());
                            }
                        };
                        let status = match status {
                            "0" => super::proto::DomainStatus::Unknown,
                            "2" => super::proto::DomainStatus::RegisteredUntilExpiry,
                            "4" => super::proto::DomainStatus::RenewalRequired,
                            "7" => super::proto::DomainStatus::NoLongerRequired,
                            _ => {
                                error!("Invalid syntax from {}: invalid status", host);
                                return Err(());
                            }
                        };

                        Ok(super::proto::DACResponse::DomainTD(
                            super::proto::DomainTD {
                                domain: domain.to_string(),
                                registered,
                                detagged,
                                suspended,
                                created,
                                expiry,
                                status,
                                tag: tag.to_string(),
                            },
                        ))
                    }
                    _ => {
                        error!("Invalid syntax from {}: expected 2, 6 or 8 parts for a domain response", host);
                        Err(())
                    }
                }
            }
        }
    }
}

async fn recv_msg<R: std::marker::Unpin + tokio::io::AsyncBufRead>(
    lines: &mut tokio::io::Lines<R>,
    host: &str,
) -> Result<super::proto::DACResponse, bool> {
    let line = match lines.next_line().await {
        Ok(Some(l)) => l,
        Ok(None) => {
            warn!("{} has closed the connection", host);
            return Err(true);
        }
        Err(err) => {
            return Err(match err.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    warn!("{} has closed the connection", host);
                    true
                }
                _ => {
                    error!("Error reading next data line from {}: {}", host, err);
                    false
                }
            });
        }
    };
    debug!("Received message from {} with contents: {}", host, line);
    let msg = decode_line(&line, host).map_err(|_| false)?;
    Ok(msg)
}

/// Tokio task that attemps to read in messages and push them onto a tokio channel as received.
pub(super) struct ClientReceiver<R: std::marker::Unpin + tokio::io::AsyncBufRead> {
    /// Host name for error reporting
    pub host: String,
    /// Read half of the TCP stream used to connect to the server
    pub reader: R,
    pub metrics_registry: crate::metrics::ScopedMetrics,
}

impl<R: 'static + std::marker::Unpin + tokio::io::AsyncBufRead + std::marker::Send>
    ClientReceiver<R>
{
    /// Starts the tokio task, and returns the receiving end of the channel to read messages from.
    pub fn run(self) -> futures::channel::mpsc::Receiver<Result<super::proto::DACResponse, bool>> {
        let (mut sender, receiver) =
            futures::channel::mpsc::channel::<Result<super::proto::DACResponse, bool>>(16);
        tokio::spawn(async move {
            let mut lines = self.reader.lines();
            loop {
                let msg = recv_msg(&mut lines, &self.host).await;
                self.metrics_registry.response_received();
                let is_close = if let Err(c) = &msg { *c } else { false };
                match sender.send(msg).await {
                    Ok(_) => {}
                    Err(_) => break,
                }
                if is_close {
                    break;
                }
            }
        });
        receiver
    }
}
