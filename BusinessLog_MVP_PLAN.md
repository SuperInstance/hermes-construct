# BusinessLog MVP Plan
## Business Management with Minimal Setup

**Domain:** https://businesslog.ai
**Repository:** https://github.com/SuperInstance/BusinessLog
**Version:** 0.1.0 MVP
**Last Updated:** January 1, 2026

---

## Executive Summary

### Problem Statement

Small business owners, freelancers, and solopreneurs face a fragmented tool landscape:

- **Accounting Overkill:** QuickBooks and Xero are powerful but complex for simple needs
- **Tool Fatigue:** Juggling 5+ separate apps (invoicing, accounting, task management, content creation)
- **AI Scattered:** AI tools exist but aren't integrated into business workflows
- **Setup Friction:** Most business tools require hours of configuration before first use
- **Cost Creep:** Subscribing to multiple specialized tools adds up quickly

**The Result:** Small businesses spend 30%+ of admin time on tool management instead of business growth.

### Solution: BusinessLog

BusinessLog is a **unified business command center** with AI-first design that gets you from zero to productive in under 5 minutes.

**Core Philosophy:**
- Minimal setup (create account → start using)
- AI-assisted workflows (let AI handle the boring stuff)
- Modular architecture (use what you need)
- Fair pricing (one tool, not five)

### Target Audience

| Segment | Size | Pain Points | BusinessLog Value |
|---------|------|-------------|-------------------|
| **Freelancers** | 59M in US | Invoicing, expense tracking, time tracking | Quick invoice generation, auto-categorize expenses |
| **Solopreneurs** | 15M+ in US | Content creation, task juggling, basic books | AI content assistant, simple task/biz mgmt |
| **Micro-businesses** (1-5 employees) | 27M+ in US | Need team coordination but not enterprise tools | Shared workspace, basic collaboration |
| **Agency Owners** | 2M+ in US | Client management, project tracking, invoicing | Client portal, project templates |

**Total Addressable Market:** 100M+ small businesses and freelancers globally

### Differentiation

| Feature | QuickBooks | FreshBooks | BusinessLog |
|---------|-----------|------------|-------------|
| **Setup Time** | 30-60 minutes | 20-30 minutes | **< 5 minutes** |
| **AI Integration** | Basic (insights only) | None | **Deep (content + workflow)** |
| **Content Creation** | None | None | **Built-in AI assistant** |
| **Task Management** | None | None | **Integrated** |
| **Learning Curve** | Steep | Moderate | **Flat** |
| **Modularity** | All-or-nothing | All-or-nothing | **Use what you need** |
| **Price** | $30-200/mo | $15-50/mo | **$15-49/mo** (see pricing section) |

---

## Core MVP Features (v0.1.0)

### MVP Scope Definition

**BusinessLog v0.1.0** focuses on three critical business workflows:

1. **Get Paid** - Invoicing and income tracking
2. **Track Spending** - Expense categorization and reporting
3. **Create Content** - AI-assisted marketing content

What's NOT in MVP (deferred to v0.2.0+):
- Payroll
- Advanced inventory
- Multi-user permissions (basic sharing only)
- Tax filing (export to tax software only)
- Time tracking

---

### Feature 1: Income & Expense Tracking

**User Story:** As a freelancer, I want to quickly log income and expenses without complex accounting jargon, so I can see my profit at a glance.

#### Core Capabilities

1. **Quick Transaction Entry**
   ```typescript
   - Mobile-friendly single-screen entry
   - Smart defaults (last category, frequent descriptions)
   - Auto-categorization using AI
   - Bank import (plaid integration - v0.1.5 stretch goal)
   ```

2. **Auto-Categorization with AI**
   - Analyze transaction descriptions
   - Suggest categories with confidence score
   - Learn from user corrections
   - Handle edge cases (e.g., "Uber for client meeting" → Travel → Client: ABC)

3. **Simple Dashboard**
   - Month-to-date income/expenses
   - Profit/loss snapshot
   - Cash flow trend (last 30 days)
   - Category breakdown

4. **Basic Reports**
   - Profit & Loss statement (monthly, quarterly, YTD)
   - Expense by category
   - Income by source/client
   - Export to CSV/PDF

#### Technical Implementation

```typescript
// Data Model
interface Transaction {
  id: string;
  type: 'income' | 'expense';
  amount: number;
  currency: 'USD' | 'EUR' | 'GBP';
  date: Date;
  category: string;
  subcategory?: string;
  description: string;
  clientId?: string;
  invoiceId?: string;
  status: 'pending' | 'cleared' | 'reconciled';
  attachments?: string[]; // Receipt URLs
  metadata: {
    aiCategoryConfidence: number;
    autoCategorized: boolean;
    source: 'manual' | 'import' | 'invoice';
  };
}

interface Category {
  id: string;
  name: string;
  type: 'income' | 'expense';
  subcategories?: string[];
  budgetLimit?: number;
  taxDeductible: boolean;
}
```

#### Default Categories (Small Business Standard)

**Income Categories:**
- Services (Consulting, Design, Development, Writing)
- Products (Sales, Subscriptions)
- Other (Refunds, Rebates)

**Expense Categories:**
- Cost of Goods Sold
- Software & Tools
- Marketing & Advertising
- Travel & Transportation
- Office Expenses
- Professional Services (Legal, Accounting)
- Insurance
- Payroll & Contractors (v0.2.0)
- Taxes & Licenses
- Other

---

### Feature 2: Invoice Generation & Management

**User Story:** As a freelancer, I want to generate professional invoices in under 60 seconds and get paid faster, so I can maintain cash flow and focus on billable work.

#### Core Capabilities

1. **Rapid Invoice Creation**
   - Client selector (saved clients)
   - Line item builder (quantity × rate)
   - Auto-fill from previous invoices
   - Customizable templates (3 professional designs)
   - Logo upload + brand colors
   - Multiple currencies
   - Tax calculation (auto or manual)

2. **Invoice Workflow**
   - Save as draft
   - Send directly from app (email)
   - Generate shareable PDF link
   - Payment reminders (automated)
   - Track status (draft → sent → viewed → paid → overdue)

3. **AI Enhancements**
   - "Generate invoice from work description"
   - Auto-suggest line items from notes
   - Professional payment reminder emails
   - Client-specific terms (learned from history)

4. **Payment Integration (MVP: Manual tracking; v0.1.5: Stripe)**
   - Record payments
   - Partial payment support
   - Payment method tracking
   - Auto-mark as paid

#### Technical Implementation

```typescript
interface Invoice {
  id: string;
  invoiceNumber: string; // Auto-generated: INV-YYYY-NNN
  clientId: string;
  status: 'draft' | 'sent' | 'viewed' | 'paid' | 'overdue' | 'void';
  date: Date;
  dueDate: Date;
  lineItems: InvoiceLineItem[];
  subtotal: number;
  taxRate: number;
  taxAmount: number;
  total: number;
  currency: string;
  notes?: string;
  terms?: string;
  logoUrl?: string;
  brandColor?: string;
  template: 'modern' | 'classic' | 'minimal';
  payments: Payment[];
  metadata: {
    sentDate?: Date;
    viewedDate?: Date;
    paidDate?: Date;
    reminderCount: number;
    shareUrl?: string;
  };
}

interface InvoiceLineItem {
  id: string;
  description: string;
  quantity: number;
  unitPrice: number;
  taxRate?: number;
  total: number;
}

interface Client {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  company?: string;
  address?: Address;
  billingEmail?: string; // Separate from contact email
  currency: string;
  defaultTerms?: string;
  notes?: string;
  metadata: {
    totalInvoiced: number;
    totalPaid: number;
    outstandingBalance: number;
    invoiceCount: number;
    lastInvoiceDate?: Date;
    averagePaymentDays: number; // Calculated from history
  };
}
```

---

### Feature 3: AI Content Creation

**User Story:** As a small business owner, I want AI to help me create marketing content, so I can maintain an online presence without spending hours writing or hiring an agency.

#### Core Capabilities

1. **Content Templates**
   - Social media posts (LinkedIn, Twitter/X, Instagram, Facebook)
   - Email newsletters
   - Blog post outlines + drafts
   - Product descriptions
   - Ad copy (Google, Facebook)
   - Landing page headlines

2. **AI Assistant Features**
   - Tone selector (professional, casual, playful, urgent)
   - Brand voice training (learn from past content)
   - SEO keywords integration
   - Hashtag generation
   - A/B testing variations (3 versions)
   - Content calendar suggestions

3. **Workflow Integration**
   - Save to content library
   - Schedule for later (integrations planned v0.2.0)
   - Export to CSV/PDF
   - Direct copy to clipboard
   - Share to social (planned v0.1.5)

4. **Business Context Awareness**
   - Pull from invoice history (e.g., "announce new service")
   - Reference past content
   - Industry-specific templates
   - Multi-language support

#### Technical Implementation

```typescript
interface ContentPiece {
  id: string;
  type: 'social-post' | 'email' | 'blog' | 'ad-copy' | 'product-description';
  platform?: 'linkedin' | 'twitter' | 'instagram' | 'facebook' | 'google-ads';
  status: 'draft' | 'scheduled' | 'published';
  title?: string;
  content: string;
  variations?: ContentVariation[];
  metadata: {
    aiModel: string;
    promptUsed: string;
    tone: string;
    targetAudience?: string;
    keywords?: string[];
    hashtags?: string[];
    scheduledDate?: Date;
    publishedDate?: Date;
    engagement?: {
      likes: number;
      shares: number;
      clicks: number;
      impressions: number;
    };
  };
}

interface ContentVariation {
  id: string;
  label: string; // e.g., "More playful", "Shorter", "Professional"
  content: string;
}

interface ContentCalendar {
  id: string;
  date: Date;
  contentPieces: string[]; // IDs
  theme?: string;
  notes?: string;
}
```

#### AI Prompt Templates

```typescript
const CONTENT_PROMPTS = {
  linkedin: `You are a social media expert for {industry}. Write a LinkedIn post about {topic}.
  Tone: {tone}
  Target audience: {audience}
  Include: {call_to_action}
  Format: 3-5 sentences, 2-3 hashtags
  `,

  blogOutline: `You are a content strategist. Create a detailed blog post outline about {topic}.
  Industry: {industry}
  Target audience: {audience}
  SEO keywords: {keywords}
  Output: Title, introduction, 4-6 main sections with bullet points, conclusion
  `,

  productDescription: `You are a copywriter. Write a compelling product description for {product_name}.
  Key features: {features}
  Benefits: {benefits}
  Target audience: {audience}
  Tone: {tone}
  Format: 100-150 words, scannable with subheadings
  `
};
```

---

### Feature 4: Basic Task & Project Management

**User Story:** As a solopreneur, I want to track business tasks alongside financials, so I have a single source of truth for my business operations.

#### Core Capabilities

1. **Simple Task Tracking**
   - Quick add from anywhere
   - Due dates and reminders
   - Priority levels (none, low, medium, high, urgent)
   - Tags/labels for organization
   - One-click complete with archive

2. **Project Grouping**
   - Bundle related tasks into projects
   - Project status (planning, active, on hold, completed)
   - Link projects to clients
   - Track billable vs. non-billable time (v0.2.0)

3. **Views**
   - Today's tasks (default home)
   - Upcoming (next 7 days)
   - By project
   - By priority
   - Completed history

4. **Integrations**
   - Tasks linked to invoices (e.g., "Send invoice to Client X")
   - Tasks linked to expenses (e.g., "Reimbursement for office supplies")
   - AI task suggestions from conversations

#### Technical Implementation

```typescript
interface Task {
  id: string;
  title: string;
  description?: string;
  status: 'todo' | 'in-progress' | 'completed' | 'archived';
  priority: 'none' | 'low' | 'medium' | 'high' | 'urgent';
  dueDate?: Date;
  completedDate?: Date;
  projectId?: string;
  clientId?: string;
  tags: string[];
  metadata: {
    created: Date;
    modified: Date;
    estimatedMinutes?: number;
    billable: boolean;
    invoiceId?: string; // If task resulted in invoice
    recurring?: RecurringRule;
  };
}

interface Project {
  id: string;
  name: string;
  description?: string;
  status: 'planning' | 'active' | 'on-hold' | 'completed';
  clientId?: string;
  color?: string; // For visual organization
  startDate?: Date;
  endDate?: Date;
  metadata: {
    taskCount: number;
    completedTasks: number;
    totalBudget?: number;
    totalInvoiced?: number;
  };
}

interface RecurringRule {
  frequency: 'daily' | 'weekly' | 'monthly' | 'yearly';
  interval: number; // e.g., every 2 weeks
  endDate?: Date;
  lastGenerated: Date;
}
```

---

## Technical Architecture

### Technology Stack

#### Frontend
```json
{
  "framework": "Next.js 15.3",
  "ui": "React 19.0",
  "language": "TypeScript 5.9",
  "styling": "Tailwind CSS 4.0",
  "icons": "Lucide React",
  "state": "React Context + Hooks",
  "forms": "React Hook Form + Zod validation",
  "tables": "TanStack Table",
  "charts": "Recharts",
  "date": "date-fns"
}
```

#### Backend
```json
{
  "runtime": "Node.js 20 LTS",
  "api": "Next.js API Routes (App Router)",
  "database": "PostgreSQL 16 (via Supabase)",
  "orm": "Drizzle ORM",
  "cache": "Redis 7 (Upstash for managed)",
  "auth": "NextAuth.js v5",
  "storage": "Supabase Storage (receipts, logos)",
  "queue": "BullMQ (background jobs)",
  "email": "Resend (transactional emails)"
}
```

#### AI/ML
```json
{
  "primary": "@superinstance/chat",
  "llmProviders": ["OpenAI", "Anthropic"],
  "contentAnalysis": "@superinstance/content-analyzer",
  "personalization": "@superinstance/personalization",
  "fallback": "Local models via Ollama (optional)"
}
```

---

### System Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                         BusinessLog                              │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    Frontend (Next.js)                    │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐  │   │
│  │  │ Dashboard│  │Financials│  │ Invoices │  │Content │  │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └─────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                            ↓↑                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    API Layer                              │   │
│  │  /api/transactions  /api/invoices  /api/content          │   │
│  └──────────────────────────────────────────────────────────┘   │
│                            ↓↑                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              SuperInstance Packages                       │   │
│  │  ┌──────────────┐  ┌──────────────────┐                 │   │
│  │  │ @si/chat     │  │ @si/content-analyzer│               │   │
│  │  └──────────────┘  └──────────────────┘                 │   │
│  └──────────────────────────────────────────────────────────┘   │
│                            ↓↑                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    Data Layer                             │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │ PostgreSQL   │  │    Redis     │  │ Supabase     │   │   │
│  │  │ (Drizzle)    │  │    (Cache)   │  │ Storage      │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

### Database Schema

```sql
-- Clients
CREATE TABLE clients (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255),
  company VARCHAR(255),
  address JSONB,
  currency VARCHAR(3) DEFAULT 'USD',
  terms TEXT,
  notes TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Transactions
CREATE TABLE transactions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type VARCHAR(10) NOT NULL CHECK (type IN ('income', 'expense')),
  amount DECIMAL(12, 2) NOT NULL,
  currency VARCHAR(3) DEFAULT 'USD',
  date DATE NOT NULL,
  category VARCHAR(100) NOT NULL,
  subcategory VARCHAR(100),
  description TEXT NOT NULL,
  client_id UUID REFERENCES clients(id) ON DELETE SET NULL,
  invoice_id UUID REFERENCES invoices(id) ON DELETE SET NULL,
  status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'cleared', 'reconciled')),
  attachments TEXT[],
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Invoices
CREATE TABLE invoices (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  invoice_number VARCHAR(50) UNIQUE NOT NULL,
  client_id UUID NOT NULL REFERENCES clients(id) ON DELETE RESTRICT,
  status VARCHAR(20) DEFAULT 'draft' CHECK (status IN ('draft', 'sent', 'viewed', 'paid', 'overdue', 'void')),
  date DATE NOT NULL DEFAULT CURRENT_DATE,
  due_date DATE NOT NULL,
  subtotal DECIMAL(12, 2) NOT NULL,
  tax_rate DECIMAL(5, 2) DEFAULT 0,
  tax_amount DECIMAL(12, 2) DEFAULT 0,
  total DECIMAL(12, 2) NOT NULL,
  currency VARCHAR(3) DEFAULT 'USD',
  notes TEXT,
  terms TEXT,
  logo_url TEXT,
  brand_color VARCHAR(7),
  template VARCHAR(20) DEFAULT 'modern',
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Invoice Line Items
CREATE TABLE invoice_line_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
  description TEXT NOT NULL,
  quantity DECIMAL(10, 2) NOT NULL DEFAULT 1,
  unit_price DECIMAL(12, 2) NOT NULL,
  tax_rate DECIMAL(5, 2) DEFAULT 0,
  total DECIMAL(12, 2) NOT NULL,
  sort_order INTEGER DEFAULT 0,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Payments
CREATE TABLE payments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
  amount DECIMAL(12, 2) NOT NULL,
  payment_date DATE NOT NULL DEFAULT CURRENT_DATE,
  method VARCHAR(50),
  notes TEXT,
  reference VARCHAR(255),
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Projects
CREATE TABLE projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  status VARCHAR(20) DEFAULT 'planning' CHECK (status IN ('planning', 'active', 'on-hold', 'completed')),
  client_id UUID REFERENCES clients(id) ON DELETE SET NULL,
  color VARCHAR(7),
  start_date DATE,
  end_date DATE,
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tasks
CREATE TABLE tasks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title VARCHAR(500) NOT NULL,
  description TEXT,
  status VARCHAR(20) DEFAULT 'todo' CHECK (status IN ('todo', 'in-progress', 'completed', 'archived')),
  priority VARCHAR(10) DEFAULT 'none' CHECK (priority IN ('none', 'low', 'medium', 'high', 'urgent')),
  due_date TIMESTAMPTZ,
  completed_at TIMESTAMPTZ,
  project_id UUID REFERENCES projects(id) ON DELETE SET NULL,
  client_id UUID REFERENCES clients(id) ON DELETE SET NULL,
  tags TEXT[],
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Content Library
CREATE TABLE content_pieces (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type VARCHAR(50) NOT NULL,
  platform VARCHAR(50),
  status VARCHAR(20) DEFAULT 'draft',
  title VARCHAR(500),
  content TEXT NOT NULL,
  variations JSONB,
  metadata JSONB DEFAULT '{}',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Categories
CREATE TABLE categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name VARCHAR(100) NOT NULL,
  type VARCHAR(10) NOT NULL CHECK (type IN ('income', 'expense')),
  subcategories TEXT[],
  budget_limit DECIMAL(12, 2),
  tax_deductible BOOLEAN DEFAULT false,
  color VARCHAR(7),
  is_custom BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_transactions_user_date ON transactions(user_id, date DESC);
CREATE INDEX idx_transactions_type ON transactions(type);
CREATE INDEX idx_transactions_category ON transactions(category);
CREATE INDEX idx_invoices_user_status ON invoices(user_id, status);
CREATE INDEX idx_invoices_client ON invoices(client_id);
CREATE INDEX idx_tasks_user_status ON tasks(user_id, status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_content_user_type ON content_pieces(user_id, type);
```

---

### API Endpoints

#### Authentication
```
POST /api/auth/register
POST /api/auth/login
POST /api/auth/logout
GET  /api/auth/session
POST /api/auth/reset-password
```

#### Transactions
```
GET    /api/transactions          # List with filters
POST   /api/transactions          # Create
GET    /api/transactions/:id      # Get one
PUT    /api/transactions/:id      # Update
DELETE /api/transactions/:id      # Delete
POST   /api/transactions/import   # Bank import (future)
GET    /api/transactions/reports  # P&L, summaries
```

#### Invoices
```
GET    /api/invoices              # List
POST   /api/invoices              # Create
GET    /api/invoices/:id          # Get one
PUT    /api/invoices/:id          # Update
DELETE /api/invoices/:id          # Delete
POST   /api/invoices/:id/send     # Send to client
GET    /api/invoices/:id/pdf      # Download PDF
POST   /api/invoices/:id/payments # Record payment
GET    /api/invoices/next-number  # Auto-generate invoice #
```

#### Clients
```
GET    /api/clients               # List
POST   /api/clients               # Create
GET    /api/clients/:id           # Get one
PUT    /api/clients/:id           # Update
DELETE /api/clients/:id           # Delete (with checks)
GET    /api/clients/:id/invoices  # Related invoices
```

#### Content
```
GET    /api/content               # List library
POST   /api/content/generate      # AI generate
GET    /api/content/:id           # Get one
PUT    /api/content/:id           # Update
DELETE /api/content/:id           # Delete
POST   /api/content/:id/variations # Generate alternatives
GET    /api/content/templates     # List templates
```

#### Tasks & Projects
```
GET    /api/tasks                 # List with filters
POST   /api/tasks                 # Create
GET    /api/tasks/:id             # Get one
PUT    /api/tasks/:id             # Update
DELETE /api/tasks/:id             # Delete
POST   /api/tasks/:id/complete    # Mark complete

GET    /api/projects              # List
POST   /api/projects              # Create
GET    /api/projects/:id          # Get one
PUT    /api/projects/:id          # Update
DELETE /api/projects/:id          # Delete
GET    /api/projects/:id/tasks    # Project tasks
```

#### Reports
```
GET    /api/reports/profit-loss   # P&L statement
GET    /api/reports/expense-breakdown
GET    /api/reports/income-sources
GET    /api/reports/cash-flow
GET    /api/reports/client-summary
```

---

## User Stories & Workflows

### User Story 1: First-Time Freelancer Setup

**Persona:** Sarah, freelance graphic designer, just started her business

**Scenario:**
1. Sarah signs up for BusinessLog (2 minutes)
2. Onboarding asks 3 questions:
   - "What do you do?" (Designer)
   - "How do you get paid?" (Invoicing clients)
   - "What's your biggest pain?" (Keeping track of expenses)
3. Dashboard is pre-configured with relevant categories
4. Sarah creates her first invoice in 3 minutes

**Steps:**
```typescript
// 1. Sign up
POST /api/auth/register
{
  "email": "sarah@example.com",
  "password": "securepassword",
  "name": "Sarah Design",
  "businessType": "freelancer",
  "industry": "design"
}

// 2. Create first client
POST /api/clients
{
  "name": "Acme Corp",
  "email": "billing@acme.com",
  "company": "Acme Corp",
  "currency": "USD"
}

// 3. Generate first invoice
POST /api/invoices
{
  "clientId": "uuid",
  "lineItems": [
    {
      "description": "Logo design",
      "quantity": 1,
      "unitPrice": 1500
    }
  ],
  "dueDate": "2026-02-01",
  "notes": "Thank you for your business!"
}

// 4. Send invoice
POST /api/invoices/:id/send
{
  "method": "email"
}
```

**Time to First Value:** < 10 minutes

---

### User Story 2: Expense Tracking on the Go

**Persona:** Mike, consultant, travels frequently

**Scenario:**
1. Mike buys coffee for a client meeting ($45)
2. Opens BusinessLog mobile app
3. Takes photo of receipt
4. System auto-categorizes as "Travel > Client Meals"
5. Mike confirms with one tap
6. Transaction is logged and receipt stored

**AI Enhancement:**
- Description: "Starbucks - client meeting with John from Acme"
- Auto-suggest: Category = "Travel", Subcategory = "Client Meals", Client = "Acme"
- Confidence: 94%
- Action: One-tap confirm

---

### User Story 3: Monthly Invoicing Batch

**Persona:** Maria, web development agency owner

**Scenario:**
1. Maria completed work for 5 clients this month
2. She goes to Invoices → "Batch Create"
3. Selects clients + pre-built line items from templates
4. AI suggests amounts based on time tracking notes
5. Maria reviews and adjusts
6. Clicks "Send All" (5 invoices sent in 2 minutes)

**Time Saved:** 30+ minutes vs. manual creation

---

### User Story 4: AI Content Creation

**Persona:** James, solopreneur with online course business

**Scenario:**
1. James wants to promote his new course on LinkedIn
2. Opens BusinessLog → Content → Create New
3. Selects:
   - Platform: LinkedIn
   - Topic: "Launch of Productivity Masterclass"
   - Tone: Professional but approachable
   - CTA: "Join waitlist"
4. AI generates 3 variations
5. James edits variation 2 slightly
6. Copies to clipboard and posts
7. Saves to content library for future reference

**Output Example:**
```
Variation 1 (Professional):
🚀 Exciting news! I'm launching "Productivity Masterclass" - a 4-week program
to help entrepreneurs reclaim 10+ hours per week.

After coaching 100+ business owners, I've distilled the strategies that
actually work. No fluff, just actionable systems.

🎯 What you'll learn:
• Time blocking that sticks
• Email inbox zero (forever)
• Task prioritization frameworks

Early bird pricing ends Friday! [Link]

Variation 2 (Story-driven):
I used to work 60-hour weeks and feel like I accomplished nothing.

Then I discovered something that changed everything: productivity isn't
about doing more—it's about doing what matters.

I packaged everything I learned into "Productivity Masterclass."

If you're tired of busywork and ready for progress, join the waitlist:
[Link]

Variation 3 (Direct):
New course alert: "Productivity Masterclass" 👩‍🏫

For entrepreneurs who:
→ Start every morning overwhelmed
→ Never feel "caught up"
→ Want to grow without burnout

4 weeks. Actionable systems. Lasting results.

Doors open next week. Get on the list: [Link]
```

**Time Saved:** 45+ minutes vs. writing from scratch

---

### User Story 5: End-of-Month Review

**Persona:** Lisa, freelance marketing consultant

**Scenario:**
1. Lisa logs into BusinessLog on the 1st of the month
2. Dashboard shows:
   - Total income: $8,450
   - Total expenses: $2,120
   - Net profit: $6,330
   - 2 invoices overdue
   - 5 tasks due this week
3. She clicks "Reports → Profit & Loss"
4. Exports PDF for her accountant
5. Sends payment reminders for overdue invoices (one click)
6. Reviews expense categories
7. Notes: "Software expenses up 40% - need to review subscriptions"

**Outcome:** 10-minute monthly review vs. 2+ hours with spreadsheets

---

## Differentiation Strategy

### Why BusinessLog Over Competitors?

#### 1. Minimal Setup Philosophy

| Feature | QuickBooks | FreshBooks | **BusinessLog** |
|---------|-----------|------------|-----------------|
| Setup wizard | 15+ steps | 10+ steps | **3 questions** |
| Required config | Company details, tax, chart of accounts, bank feeds | Company, tax, invoice template | **Name + email** |
| Time to first invoice | 20-30 min | 15-20 min | **< 5 min** |
| Learning curve | Steep (accounting terms) | Moderate | **Flat (plain language)** |

#### 2. AI-Native vs. AI-Added

**Traditional Tools (AI as feature):**
- Basic expense categorization
- Invoice reminders
- Cash flow predictions
- Added in 2023-2024 as bolt-on

**BusinessLog (AI-first design):**
- Content generation core to product
- Conversational task creation
- Intelligent invoice suggestions
- Brand voice learning
- Built on @superinstance/chat from ground up

#### 3. Unified vs. Fragmented

**Competitor Approach:**
- Accounting software (QuickBooks)
- Project management (Asana/Trello)
- Content creation (ChatGPT, Jasper)
- Time tracking (Toggl)
- Invoicing (separate module of accounting)

**BusinessLog Approach:**
```
┌─────────────────────────────────────┐
│         BusinessLog                 │
│  ┌──────────┬──────────┬──────────┐  │
│  │Financial│ Tasks    │ Content  │  │
│  │          │          │          │  │
│  │• Track   │• Manage  │• Create  │  │
│  │• Invoice │• Organize│• Schedule│  │
│  └──────────┴──────────┴──────────┘  │
│           All Integrated              │
└─────────────────────────────────────┘
```

**Benefits:**
- Single subscription ($15-49/mo vs. $50-200/mo for multiple tools)
- Shared context (tasks linked to invoices, content linked to clients)
- One login, one interface to learn
- Data flows seamlessly between modules

#### 4. Developer & Power User Friendly

**Open Source Foundation:**
- Community-driven feature development
- Self-hosting option (planned v0.3.0)
- API-first design
- Extensible via plugins (planned v0.4.0)

#### 5. Fair, Transparent Pricing

| Plan | QuickBooks | FreshBooks | **BusinessLog** |
|------|-----------|------------|-----------------|
| Starter | $30/mo | $15/mo | **$15/mo** |
| Professional | $90/mo | $50/mo | **$29/mo** |
| Unlimited/Plus | $200/mo | N/A | **$49/mo** |

**BusinessLog Advantage:**
- All features included (no gated modules)
- Unlimited invoices on all plans
- Unlimited AI generations (fair use policy)
- No per-user fees (up to 5 users on Pro)

---

## Monetization Strategy

### Pricing Model

#### Free Tier (v0.2.0 - lead magnet)
- 5 invoices/month
- 25 transactions/month
- 10 AI content generations/month
- 1 user
- Community support
- **Purpose:** Try before you buy, hook users

#### Starter - $15/month
- **Target:** Freelancers, solopreneurs
- **Limits:**
  - 25 invoices/month
  - 100 transactions/month
  - 100 AI content generations/month
  - 5 projects
  - 1 user
- **Features:** All core features
- **Support:** Email

#### Professional - $29/month ⭐️ Best Value
- **Target:** Established freelancers, micro-businesses
- **Limits:**
  - Unlimited invoices
  - Unlimited transactions
  - 500 AI content generations/month
  - Unlimited projects
  - 3 users
  - Priority support
- **Features:**
  - Everything in Starter
  - Custom invoice templates
  - Client portal
  - Advanced reports
  - API access (read-only)

#### Business - $49/month
- **Target:** Agencies, teams
- **Limits:**
  - Everything unlimited
  - Unlimited AI (fair use)
  - 10 users
  - Dedicated support
- **Features:**
  - Everything in Professional
  - White-label invoice emails
  - Custom domains
  - Webhooks
  - Bulk operations
  - Team permissions
  - SSO (v0.3.0)

#### Enterprise - Custom
- **Target:** Large organizations
- **Features:**
  - Unlimited users
  - Custom contracts
  - SLA guarantees
  - On-premise deployment
  - Custom integrations

---

### Revenue Projections

**Year 1 (2026):**
- Goal: 1,000 paying customers
- Mix: 60% Starter, 30% Pro, 10% Business
- Average revenue: $24/user
- **ARR: $288,000**

**Year 2 (2027):**
- Goal: 5,000 paying customers
- Mix: 50% Starter, 35% Pro, 15% Business
- Average revenue: $26/user
- **ARR: $1.56M**

**Year 3 (2028):**
- Goal: 15,000 paying customers
- Add self-hosted licenses ($999/yr)
- Enterprise deals
- **ARR: $5M+**

---

### Go-to-Market Strategy

#### Phase 1: MVP Launch (Q1 2026)
- Target: SuperInstance community early adopters
- Channels: GitHub, HackerNews, ProductHunt
- Offer: Lifetime discount for first 500 users
- Goal: 100 beta users, feedback collection

#### Phase 2: Content Marketing (Q2-Q3 2026)
- Publish "AI for Small Business" guides
- YouTube channel (tutorials, tips)
- LinkedIn thought leadership
- SEO: "business management software", "AI invoicing"
- Goal: 500 paying customers

#### Phase 3: Partnerships (Q4 2026)
- Integrate with popular tools (Stripe, Notion, Slack)
- Partner with business coach networks
- Affiliate program (20% commission)
- Goal: 1,500 paying customers

#### Phase 4: Paid Acquisition (2027)
- Google Ads: "QuickBooks alternative", "invoice generator"
- Sponsor podcasts (entrepreneurship, small business)
- Conference presence (MicroConf, AltSummit)
- Goal: Scale to 5,000+ customers

---

## Development Roadmap

### v0.1.0 - MVP (Target: March 2026)

**Core Features:**
- ✅ User authentication (NextAuth.js)
- ✅ Client management (CRUD)
- ✅ Transaction tracking (income/expense)
- ✅ Invoice generation + PDF
- ✅ AI content creation (3 templates)
- ✅ Basic task management
- ✅ Dashboard with key metrics
- ✅ P&L report

**Success Criteria:**
- 5-minute onboarding flow
- Create invoice in under 60 seconds
- AI content generation < 10 seconds
- 99.9% API uptime (during beta)
- 50 beta users complete "first invoice" flow

---

### v0.1.5 - Polish + Integration (Target: May 2026)

**Features:**
- Bank feed integration (Plaid)
- Stripe payment collection on invoices
- Email improvements (SMTP configuration)
- Mobile responsive improvements
- Onboarding wizard (3-question flow)
- Dark mode
- Import from CSV (migrate from other tools)

**Success Criteria:**
- 90% of new users complete onboarding
- < 2% mobile browser issues
- 50 users actively using weekly

---

### v0.2.0 - Advanced Features (Target: August 2026)

**Features:**
- Multi-user collaboration
- Client portal (view invoices, pay online)
- Advanced reports (cash flow, aging, budget vs. actual)
- Recurring invoices
- Time tracking (basic)
- Expense approvals workflow
- Free tier launch
- Webhooks (Pro plan)

**Success Criteria:**
- 200 paying customers
- 30% MoM growth
- NPS score > 40

---

### v0.3.0 - Ecosystem Expansion (Target: November 2026)

**Features:**
- Payroll (basic, US only)
- Inventory tracking (basic)
- Tax preparation exports (TurboTax, TaxJar)
- Accounting integrations:
  - QuickBooks sync (two-way)
  - Xero sync (two-way)
- Self-hosted option ($999/yr license)
- Plugin API
- 5 new content templates
- Brand voice training

**Success Criteria:**
- 500 paying customers
- 100 self-hosted licenses
- Integration with 2+ major accounting tools

---

### v0.4.0 - Platform Maturity (Target: February 2027)

**Features:**
- Mobile apps (iOS, Android)
- Advanced AI:
  - Generate invoices from conversation notes
  - Predict cash flow shortages
  - Auto-categorize with 95% accuracy
- White-label option
- Custom domains
- API write access
- SSO (SAML, OIDC)
- Advanced permissions (roles)
- Audit logs
- SOC 2 Type I compliance

**Success Criteria:**
- 2,000 paying customers
- $1M ARR
- 10 enterprise customers

---

### v1.0.0 - General Availability (Target: June 2027)

**Features:**
- All v0.x features stable
- 99.99% uptime SLA
- 24/7 support (Pro plans)
- Comprehensive documentation
- Public API stable
- Plugin marketplace
- Certified integrations (20+ apps)
- Multi-currency (advanced)
- Multi-language (Spanish, French, German)

**Success Criteria:**
- 5,000 paying customers
- $2M ARR
- NPS > 50
- < 1% monthly churn

---

## Risk Mitigation

### Technical Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| **AI API costs** | High - Could burn cash | 1. Caching common prompts<br>2. Local model fallback<br>3. Fair use limits<br>4. Monitor costs/user |
| **Data security breach** | Critical - Business data | 1. Encryption at rest + in transit<br>2. Regular security audits<br>3. Bug bounty program<br>4. SOC 2 compliance by v0.4 |
| **Performance degradation** | Medium - User churn | 1. Database indexing strategy<br>2. Redis caching layer<br>3. CDN for static assets<br>4. Load testing before launches |
| **Third-party API failures** | Medium - Feature broken | 1. Circuit breakers<br>2. Fallback mechanisms<br>3. Status page<br>4. Graceful degradation |

### Business Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| **Competition from incumbents** | High - Market share | 1. Focus on underserved freelancers<br>2. Faster innovation cycle<br>3. Community loyalty<br>4. Open source differentiation |
| **Price wars** | Medium - Margin pressure | 1. Value-based pricing (AI features)<br>2. Stick to fair, transparent pricing<br>3. Avoid freemium abuse<br>4. Emphasize all-in-one value |
| **Slow adoption** | High - Runway burn | 1. Agile MVP, iterate fast<br>2. Strong feedback loops<br>3. Community-driven development<br>4. Early traction metrics focus |
| **Regulatory changes** | Medium - Compliance costs | 1. Monitor accounting standards<br>2. Work with accountants<br>3. Flexible data model<br>4. Export-focused (not tax filing) |

---

## Success Metrics

### North Star Metric

**Weekly Active Users (WAU)** with ≥ 2 core actions

**Core actions:**
- Create/edit transaction
- Create/send invoice
- Generate AI content
- Complete task

### Leading Indicators (v0.1.0)

| Metric | Target | Timeline |
|--------|--------|----------|
| Beta signups | 500 | Week 2 of MVP |
| First invoices created | 250 | Week 4 of MVP |
| Retention (Week 1 → Week 4) | > 40% | Week 8 of MVP |
| AI content generations | 1,000 total | Week 12 of MVP |
| Feature requests (GitHub) | 100 unique | Week 8 of MVP |

### Lagging Indicators (v0.1.0 → v0.2.0)

| Metric | Target | Timeline |
|--------|--------|----------|
| Paying customers | 100 | End of v0.1.0 (3 months) |
| MRR | $2,400 | End of v0.1.0 |
| Customer acquisition cost (CAC) | < $50 | End of v0.1.5 |
| Lifetime value (LTV) | > $300 | End of v0.2.0 |
| Monthly churn | < 5% | End of v0.2.0 |
| Net Promoter Score (NPS) | > 30 | End of v0.2.0 |

### Qualitative Metrics

- **User satisfaction:** > 4.0/5.0 in in-app surveys
- **Support ticket volume:** < 10% of users monthly
- **Feature request response time:** < 48 hours
- **Time to value:** < 10 minutes from signup to first invoice

---

## Next Steps: MVP Execution Plan

### Week 1-2: Foundation (January 2026)
- [ ] Set up Next.js 15 project structure
- [ ] Configure Supabase (PostgreSQL, Auth, Storage)
- [ ] Set up Drizzle ORM with schema
- [ ] Implement NextAuth.js authentication
- [ ] Create base UI components (button, input, card)
- [ ] Set up Tailwind CSS 4 with design tokens

### Week 3-4: Core Data Models
- [ ] Implement clients CRUD
- [ ] Implement transactions CRUD
- [ ] Build categorization system
- [ ] Create invoice data model
- [ ] Build invoice number generation
- [ ] Implement invoice line items

### Week 5-6: Invoicing Features
- [ ] Invoice creation UI
- [ ] Invoice template system (3 designs)
- [ ] PDF generation (react-pdf or similar)
- [ ] Invoice sending (email via Resend)
- [ ] Invoice status tracking
- [ ] Payment recording

### Week 7-8: Dashboard & Reports
- [ ] Build dashboard with key metrics
- [ ] Implement P&L report
- [ ] Create expense breakdown chart
- [ ] Build income by client view
- [ ] Add date range filters
- [ ] Export to CSV/PDF

### Week 9-10: AI Content Creation
- [ ] Integrate @superinstance/chat
- [ ] Build content generation UI
- [ ] Implement template system
- [ ] Create content library
- [ ] Add tone/prompt customization
- [ ] Build variation generator

### Week 11-12: Task Management
- [ ] Implement task CRUD
- [ ] Build project management
- [ ] Create task views (today, upcoming, by project)
- [ ] Add task linking to invoices
- [ ] Implement task completion flow

### Week 13-14: Polish & Testing
- [ ] Responsive design audit
- [ ] Accessibility audit (WCAG AA)
- [ ] Performance optimization
- [ ] Security review
- [ ] Beta user onboarding
- [ ] Bug fixes

### Week 15-16: Beta Launch
- [ ] Deploy to production
- [ ] Launch landing page (businesslog.ai)
- [ ] GitHub repo public
- [ ] ProductHunt launch prep
- [ ] Gather initial feedback
- [ ] Iterate on top issues

---

## Resources & References

### Development Resources
- **Next.js 15 Docs:** https://nextjs.org/docs
- **Supabase:** https://supabase.com/docs
- **Drizzle ORM:** https://orm.drizzle.team
- **NextAuth.js:** https://authjs.dev
- **Resend (Email):** https://resend.com/docs

### Design Inspiration
- **Linear App:** https://linear.app
- **Notion:** https://notion.so
- **Raycast:** https://raycast.com
- **Lichess:** https://lichess.org (open source SaaS)

### Accounting & Business
- **QuickBooks Online API:** https://developer.intuit.com/app/developer/qbo/docs/learn/rest-api-essentials
- **Xero API:** https://developer.xero.com/documentation/api/accounting/overview
- **FreshBooks API:** https://www.freshbooks.com/api

### AI & Content
- **OpenAI API:** https://platform.openai.com/docs
- **Anthropic Claude:** https://docs.anthropic.com
- **SuperInstance Chat:** /mnt/c/users/casey/podcastz/packages/chat/

### Competitor Research
- **QuickBooks Simple Start:** https://quickbooks.intuit.com/pricing/
- **FreshBooks Lite:** https://www.freshbooks.com/pricing
- **Wave Accounting:** https://www.waveapps.com/pricing
- **Bonsai:** https://www.hellobonsai.com/pricing

---

## Conclusion

BusinessLog fills a critical gap in the small business software market:

✅ **Too complex** → Not like QuickBooks for freelancers
✅ **Too simple** → Not like spreadsheets
✅ **Too fragmented** → Not like juggling 5 apps
✅ **Just right** → Unified, AI-first, minimal setup

**The Opportunity:**
- 100M+ small businesses globally
- Growing freelancing economy
- AI adoption acceleration
- Legacy tool frustration

**The Strategy:**
1. **Launch MVP** with 4 core features (March 2026)
2. **Build community** through open source + great UX
3. **Iterate fast** based on user feedback
4. **Expand ecosystem** with integrations + plugins
5. **Scale platform** to broader SMB market

**The Vision:**
BusinessLog becomes the **operating system for small business**—where finances, tasks, and growth all live together in one beautiful, AI-powered workspace.

---

**Status:** Draft MVP Plan v0.1
**Next Review:** After MVP beta feedback (May 2026)
**Owner:** SuperInstance / BusinessLog Team

---

## Sources

- [QuickBooks Integration Guide 2025](https://www.apideck.com/blog/build-an-integration-with-quickbooks-desktop-in-2025)
- [QuickBooks Starter Integration Guide](https://coefficient.io/quickbooks-api/setup-quickbooks-api-integration)
- [Xero Developer Platform](https://developer.xero.com/)
- [Xero Accounting API Overview](https://developer.xero.com/documentation/api/accounting/overview)
- [Xero App Integrations](https://www.xero.com/accounting-software/app-integrations/)
- [Best AI Bookkeeping Tools for 2025](https://ddevi.com/blog/best-5-ai-bookkeeping-tools-for-2025)
- [Small Business Bookkeeping Software AI Picks](https://omniga.ai/blog/bookkeeping/solutions/small-business-bookkeeping-software-ai-features-costs-best-picks-2025)
- [9 Best AI Tools for Accounting in 2025](https://relayfi.com/blog/9-best-ai-tools-accounting/)
- [Top 10 AI Accounting Tools 2025](https://accountancyage.com/2025/01/24/top-10-ai-accounting-tools-every-business-needs-in-2025/)

