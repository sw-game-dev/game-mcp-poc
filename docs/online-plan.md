# Online Deployment Plan for TTTTT MCP Server

## Overview

This document outlines a plan for deploying the TTTTT (Trash Talkin' Tic-Tac-Toe) MCP server online so that AI agents from anywhere can connect and play. The deployment will host both the web UI for humans and the MCP HTTP endpoint for AI agents.

## Deployment Architecture

```
                    ┌─────────────────────────┐
                    │   Your Subdomain        │
                    │  ttttt.yourdomain.com   │
                    └──────────┬──────────────┘
                               │
                    ┌──────────▼──────────────┐
                    │   Reverse Proxy         │
                    │   (nginx/caddy)         │
                    │   - HTTPS/TLS           │
                    │   - Rate limiting       │
                    │   - CORS headers        │
                    └──────────┬──────────────┘
                               │
                    ┌──────────▼──────────────┐
                    │   Backend Server        │
                    │   (Rust/Axum)           │
                    │   - Port 7397           │
                    │   - /mcp endpoint       │
                    │   - /api/* REST         │
                    │   - /sse events         │
                    │   - Static files        │
                    └──────────┬──────────────┘
                               │
                    ┌──────────▼──────────────┐
                    │   SQLite Database       │
                    │   (Persistent volume)   │
                    │   - Game state          │
                    │   - Move history        │
                    │   - Taunts              │
                    └─────────────────────────┘
```

## Hosting Options

### Option 1: VPS (DigitalOcean, Linode, Vultr)
**Best for:** Full control, dedicated resources, production deployment

**Pros:**
- Full SSH access
- Dedicated resources
- Can run systemd services
- Easy to scale
- Direct SQLite file access

**Cons:**
- Requires server management
- More expensive ($5-10/month minimum)
- Manual setup and maintenance

**Recommended VPS Specs:**
- 1 GB RAM minimum
- 1 vCPU
- 25 GB SSD
- Cost: ~$5-6/month

### Option 2: Fly.io
**Best for:** Quick deployment, automatic scaling, low maintenance

**Pros:**
- Free tier available (3 shared-cpu-1x VMs)
- Automatic HTTPS
- Built-in persistent volumes
- Easy deployments with `flyctl`
- Great for Rust applications

**Cons:**
- Limited free tier resources
- Requires Dockerfile
- Less control than VPS

### Option 3: Railway.app
**Best for:** Simplest deployment, great DX

**Pros:**
- Very easy deployment from GitHub
- Automatic HTTPS
- Built-in persistent storage
- Free tier: $5 credit/month
- No Dockerfile needed (detects Rust)

**Cons:**
- Free tier may not be sufficient for heavy use
- Less control than VPS

### Option 4: Self-hosted (Home server + Cloudflare Tunnel)
**Best for:** Zero hosting cost, learning experience

**Pros:**
- Completely free
- Full control
- No bandwidth limits

**Cons:**
- Requires home server/Raspberry Pi
- More complex networking setup
- Dependent on home internet

## Recommended Approach: Fly.io

Fly.io is recommended for the initial deployment because:
1. Free tier is sufficient for demonstration/testing
2. Excellent Rust support
3. Simple deployment process
4. Automatic HTTPS with custom domains
5. Persistent volumes for SQLite database

## Step-by-Step Deployment Plan

### Phase 1: Preparation

#### 1.1 Create Dockerfile
Create a multi-stage Docker build:
- Stage 1: Build Rust binary (release mode)
- Stage 2: Build frontend with trunk
- Stage 3: Runtime image with binary + static files

**Key considerations:**
- Use rust:1.83-slim as builder
- Copy only necessary files to runtime
- Set GAME_DB_PATH to persistent volume
- Expose port 7397

#### 1.2 Add Health Check Endpoint
Add `/health` endpoint to backend:
- Returns 200 OK with uptime
- Used by load balancers to check service health

#### 1.3 Environment Configuration
Create `.env.example`:
```
PORT=7397
GAME_DB_PATH=/data/game.db
RUST_LOG=info
CORS_ORIGIN=https://ttttt.yourdomain.com
```

### Phase 2: Fly.io Setup

#### 2.1 Install Fly CLI
```bash
curl -L https://fly.io/install.sh | sh
```

#### 2.2 Initialize Fly App
```bash
fly launch
```
Answer prompts:
- App name: `ttttt-mcp`
- Region: Choose closest to primary users
- Database: Skip (using SQLite)
- Deploy now: No

#### 2.3 Configure Persistent Volume
```bash
fly volumes create ttttt_data --size 1 # 1GB
```

Update `fly.toml`:
```toml
[mounts]
  source = "ttttt_data"
  destination = "/data"
```

#### 2.4 Configure Environment
```bash
fly secrets set GAME_DB_PATH=/data/game.db
fly secrets set RUST_LOG=info
```

#### 2.5 Deploy
```bash
fly deploy
```

### Phase 3: Custom Domain Setup

#### 3.1 Add Domain to Fly
```bash
fly certs create ttttt.yourdomain.com
```

#### 3.2 Configure DNS
Add CNAME record at your DNS provider:
```
ttttt.yourdomain.com -> [your-app].fly.dev
```

Wait for DNS propagation (5-30 minutes).

#### 3.3 Verify HTTPS Certificate
```bash
fly certs check ttttt.yourdomain.com
```

### Phase 4: CORS and Security Configuration

#### 4.1 Update CORS Settings
Modify `backend/src/main.rs` to allow cross-origin requests:

```rust
let cors = CorsLayer::new()
    .allow_origin(std::env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "*".to_string())
        .parse::<HeaderValue>()
        .unwrap())
    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
    .allow_headers([CONTENT_TYPE, ACCEPT])
    .allow_credentials(false);
```

#### 4.2 Add Rate Limiting (Optional)
Consider adding tower-governor for rate limiting:
- Limit MCP endpoint to 60 requests/minute per IP
- Prevent abuse from automated agents

#### 4.3 Add Request Logging
Ensure all MCP requests are logged with:
- Timestamp
- IP address
- Method called
- Success/failure

### Phase 5: Monitoring and Observability

#### 5.1 Set Up Fly Monitoring
```bash
fly dashboard
```
Monitor:
- Request rate
- Response times
- Error rates
- Memory/CPU usage

#### 5.2 Log Aggregation
```bash
fly logs
```
Or set up log shipping to:
- Papertrail
- Logtail
- Better Stack

#### 5.3 Uptime Monitoring
Set up external monitoring with:
- UptimeRobot (free tier: 50 monitors)
- Checkly
- Pingdom

Check endpoints:
- `GET /health` - Should return 200
- `POST /mcp` with initialize request - Should return protocol info

### Phase 6: AI Agent Connection Documentation

#### 6.1 Update examples/README.md
Add section for production server:

```markdown
## Connecting to Production Server

The TTTTT MCP server is hosted at:
- **Web UI**: https://ttttt.yourdomain.com
- **MCP Endpoint**: https://ttttt.yourdomain.com/mcp

### OpenAI Configuration
Update MCP_URL in examples:
```python
MCP_URL = "https://ttttt.yourdomain.com/mcp"
```

### Gemini Configuration
Same as OpenAI - update the URL.

### Claude Desktop
For HTTP transport (not stdio), create custom integration:
```json
{
  "mcpServers": {
    "ttttt-online": {
      "url": "https://ttttt.yourdomain.com/mcp",
      "transport": "http"
    }
  }
}
```
Note: Claude Desktop primarily uses stdio, so HTTP transport may require wrapper.
```

#### 6.2 Create Public API Documentation
Create `docs/api.md`:
- MCP endpoint URL
- Available methods
- Request/response examples
- Rate limits
- Terms of use

#### 6.3 Add Landing Page Improvements
Update web UI to include:
- "Connect Your AI Agent" section
- Link to examples/README.md
- MCP endpoint URL display
- Live server status indicator

### Phase 7: Multi-Game Support (Optional Enhancement)

#### 7.1 Game ID in URL
Currently one global game. Consider:
- `/game/:id` routes
- Create new game: `POST /api/games`
- Join game: `GET /api/games/:id`
- MCP methods take optional `game_id` param

#### 7.2 Game Lobby
- List active games
- Filter by: waiting for players, in progress, completed
- AI agents can create/join specific games

### Phase 8: Testing Production Deployment

#### 8.1 Smoke Tests
```bash
# Test health endpoint
curl https://ttttt.yourdomain.com/health

# Test MCP initialize
curl -X POST https://ttttt.yourdomain.com/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'

# Test web UI
curl https://ttttt.yourdomain.com/

# Test SSE endpoint
curl -N https://ttttt.yourdomain.com/sse
```

#### 8.2 Load Testing
Use `wrk` or `hey` to test:
```bash
hey -n 1000 -c 10 https://ttttt.yourdomain.com/health
```

#### 8.3 AI Agent Integration Test
Run examples/openai_agent.py against production:
```bash
export OPENAI_API_KEY="sk-..."
# Update MCP_URL in script
python3 examples/openai_agent.py
```

## Security Considerations

### Data Privacy
- Game state is public (by design)
- No user authentication required
- Consider adding optional API keys for agent identification

### Abuse Prevention
- Rate limiting per IP: 60 req/min for MCP endpoint
- Max game duration: Auto-restart after 24 hours
- Database size limits: Clean up old games periodically

### DDoS Protection
- Cloudflare free tier in front of Fly.io
- Enable "I'm Under Attack" mode if needed

### Input Validation
- Already implemented: row/col bounds checking
- Already implemented: cell occupancy validation
- Consider: Maximum taunt message length (currently unlimited?)

## Cost Estimates

### Fly.io (Recommended)
- **Free tier**: 3 shared-cpu-1x VMs, 160GB bandwidth
- **If exceeded**: ~$3-5/month for single VM
- **Persistent volume**: 1GB free, then $0.15/GB/month
- **Bandwidth**: $0.02/GB after free tier
- **Total estimated cost**: $0-5/month for moderate use

### VPS Alternative
- **DigitalOcean Droplet**: $6/month (1GB RAM)
- **Cloudflare**: Free (DNS + DDoS protection)
- **Domain**: $10-15/year (if new)
- **Total**: ~$6-8/month

## Maintenance Plan

### Daily
- Monitor error rates via Fly dashboard
- Check logs for unusual activity

### Weekly
- Review game database size
- Clean up completed games older than 7 days

### Monthly
- Update Rust dependencies
- Security audit of dependencies
- Review and optimize database

### Quarterly
- Load testing
- Review and update documentation
- User feedback collection

## Rollback Plan

### If deployment fails:
1. Check Fly logs: `fly logs`
2. Review build output
3. Test Docker image locally
4. Roll back to previous version: `fly releases`

### If production has issues:
1. Immediate: Scale down to 0, fix issues
2. Quick fix: Deploy hotfix from main branch
3. Serious issue: Revert to last known good deployment

## Success Metrics

### Launch Goals (Week 1)
- [ ] Deployment successful with 99%+ uptime
- [ ] At least 3 successful AI agent connections
- [ ] Web UI accessible and functional
- [ ] SSL certificate valid
- [ ] No critical errors in logs

### Growth Goals (Month 1)
- [ ] 100+ games played
- [ ] 5+ different AI agents connected
- [ ] Documentation viewed by 50+ unique visitors
- [ ] Average response time < 200ms

### Long-term Goals (Month 3+)
- [ ] 1000+ games played
- [ ] Featured in AI agent showcase/gallery
- [ ] Community contributions (issues/PRs)
- [ ] Zero downtime deployments established

## Future Enhancements

### Phase 9: Advanced Features
1. **Multi-player support**: Multiple concurrent games
2. **Tournament mode**: Bracket-style AI competitions
3. **Leaderboard**: Track AI agent win rates
4. **Replay viewer**: Watch games via game ID
5. **WebSocket transport**: Alternative to SSE for real-time updates
6. **Agent authentication**: Optional API keys for tracking
7. **Analytics dashboard**: Games played, popular agents, etc.

### Phase 10: Community Features
1. **GitHub Pages for docs**: Host API docs statically
2. **Discord/Slack integration**: Notify on game events
3. **Twitter bot**: Share interesting game moments
4. **Agent gallery**: Showcase different AI strategies

## Timeline

### Week 1: Infrastructure Setup
- Day 1-2: Create Dockerfile, test locally
- Day 3-4: Deploy to Fly.io staging
- Day 5-6: Custom domain setup, SSL verification
- Day 7: Production deployment, smoke tests

### Week 2: Documentation and Testing
- Day 1-3: Update examples for production URL
- Day 4-5: Create API documentation
- Day 6-7: Load testing, security audit

### Week 3: Launch and Monitor
- Day 1: Public announcement
- Day 2-7: Monitor, fix issues, gather feedback

### Week 4: Iterate
- Implement most-requested features
- Optimize based on usage patterns
- Plan next phase

## Conclusion

Deploying TTTTT as a public MCP server will:
1. Demonstrate MCP protocol in production
2. Provide playground for AI agent developers
3. Showcase Rust/WASM full-stack development
4. Enable interesting AI vs AI gameplay experiments

The Fly.io deployment path offers the best balance of:
- Ease of deployment
- Cost (free tier available)
- Performance and reliability
- Developer experience

Next steps:
1. Create Dockerfile
2. Test Docker build locally
3. Deploy to Fly.io
4. Configure custom domain
5. Update documentation with production URLs
