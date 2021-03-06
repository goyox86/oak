// Copyright 2014 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Top-down unit inference analyses the evaluation context of each expression to propagate unit types.
//!
//! It prevents untypable expression to generate errors if the context does not expect the expression to construct a value other than unit.
//! The type of the expression is not modified, so one is expected to examine the context before using the expression type.
//! The calling context of the rules is `Both`. Semantics actions in an unvalued context won't be called.

use middle::typing::ast::*;
use middle::typing::ast::EvaluationContext::*;

pub fn top_down_unit_inference(grammar: &mut Grammar) {
  TopDownUnitInference::visit_rules(&mut grammar.rules);
}

struct TopDownUnitInference;

impl TopDownUnitInference
{
  fn visit_rules(rules: &mut HashMap<Ident, Rule>) {
    for (_, rule) in rules.iter_mut() {
      TopDownUnitInference::visit_rule(rule);
    }
  }

  fn visit_rule(rule: &mut Rule) {
    ContextExprVisitor::visit_expr(&mut rule.def, Both);
  }
}

struct ContextExprVisitor;

impl ContextExprVisitor
{
  fn visit_expr(expr: &mut Expression, mut context: EvaluationContext) {
    expr.context = expr.context.merge(context);
    if expr.is_unit() {
      context = UnValued;
    }
    ContextExprVisitor::visit_expr_node(&mut expr.node, context);
  }

  fn visit_expr_node(expr: &mut ExpressionNode, context: EvaluationContext) {
    match expr {
        &mut Sequence(ref mut exprs)
      | &mut Choice(ref mut exprs) => ContextExprVisitor::visit_exprs(&mut *exprs, context),
        &mut ZeroOrMore(ref mut expr)
      | &mut OneOrMore(ref mut expr)
      | &mut Optional(ref mut expr)
      | &mut SemanticAction(ref mut expr, _) => ContextExprVisitor::visit_expr(&mut *expr, context),
        &mut NotPredicate(ref mut expr)
      | &mut AndPredicate(ref mut expr) => ContextExprVisitor::visit_expr(&mut *expr, UnValued),
      _ => ()
    }
  }

  fn visit_exprs(exprs: &mut Vec<Box<Expression>>, context: EvaluationContext) {
    assert!(exprs.len() > 0);
    for expr in exprs.iter_mut() {
      ContextExprVisitor::visit_expr(&mut *expr, context);
    }
  }
}
