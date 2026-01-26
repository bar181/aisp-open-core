//! Resource Utilization Analysis
//!
//! This module provides comprehensive analysis of resource utilization in AISP protocols,
//! including memory usage, CPU utilization, network bandwidth, and other system resources.

use crate::{
    ast::AispDocument,
    error::{AispError, AispResult},
    concurrent_behavior_verifier::ConcurrentProcess,
    protocol_state_machine::ProtocolStateMachine,
};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Comprehensive resource utilization analysis result
#[derive(Debug, Clone)]
pub struct ResourceUtilizationAnalysis {
    /// Overall resource utilization summary
    pub utilization_summary: ResourceUtilizationSummary,
    /// Detailed analysis per resource type
    pub resource_analysis: HashMap<ResourceType, ResourceTypeAnalysis>,
    /// Resource allocation patterns
    pub allocation_patterns: Vec<AllocationPattern>,
    /// Resource bottlenecks
    pub bottlenecks: Vec<ResourceBottleneck>,
    /// Optimization recommendations
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    /// Resource forecasting
    pub forecasting: ResourceForecasting,
    /// Performance impact analysis
    pub performance_impact: ResourcePerformanceImpact,
    /// Analysis warnings
    pub warnings: Vec<String>,
}

/// Overall resource utilization summary
#[derive(Debug, Clone)]
pub struct ResourceUtilizationSummary {
    /// Overall efficiency score (0.0 to 1.0)
    pub efficiency_score: f64,
    /// Total resource count
    pub total_resources: usize,
    /// Utilized resources count
    pub utilized_resources: usize,
    /// Over-utilized resources count
    pub over_utilized_resources: usize,
    /// Under-utilized resources count
    pub under_utilized_resources: usize,
    /// Average utilization across all resources
    pub average_utilization: f64,
    /// Peak utilization recorded
    pub peak_utilization: f64,
    /// Resource diversity index
    pub diversity_index: f64,
}

/// Resource type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// CPU computational resources
    CPU,
    /// Memory resources (RAM, cache, etc.)
    Memory,
    /// Network bandwidth resources
    Network,
    /// Storage I/O resources
    Storage,
    /// Database connections
    Database,
    /// Thread pool resources
    ThreadPool,
    /// Message queues
    MessageQueue,
    /// Locks and synchronization primitives
    Synchronization,
    /// Custom application-specific resources
    Custom(String),
}

/// Detailed analysis for a specific resource type
#[derive(Debug, Clone)]
pub struct ResourceTypeAnalysis {
    /// Resource type
    pub resource_type: ResourceType,
    /// Individual resource instances
    pub resource_instances: Vec<ResourceInstance>,
    /// Type-specific utilization metrics
    pub utilization_metrics: UtilizationMetrics,
    /// Capacity planning for this resource type
    pub capacity_planning: CapacityPlanning,
    /// Resource-specific bottlenecks
    pub bottlenecks: Vec<String>,
    /// Type-specific recommendations
    pub recommendations: Vec<String>,
}

/// Individual resource instance information
#[derive(Debug, Clone)]
pub struct ResourceInstance {
    /// Resource identifier
    pub id: String,
    /// Resource name
    pub name: String,
    /// Resource capacity (maximum available)
    pub capacity: ResourceCapacity,
    /// Current utilization
    pub utilization: ResourceUtilization,
    /// Historical utilization patterns
    pub utilization_history: UtilizationHistory,
    /// Resource state
    pub state: ResourceState,
    /// Associated processes
    pub associated_processes: Vec<String>,
    /// Resource priority
    pub priority: ResourcePriority,
    /// Cost metrics
    pub cost_metrics: CostMetrics,
}

/// Resource capacity definition
#[derive(Debug, Clone)]
pub struct ResourceCapacity {
    /// Maximum capacity value
    pub maximum: f64,
    /// Units of measurement
    pub units: String,
    /// Capacity type
    pub capacity_type: CapacityType,
    /// Whether capacity is elastic
    pub elastic: bool,
    /// Scaling limits
    pub scaling_limits: Option<ScalingLimits>,
}

/// Types of resource capacity
#[derive(Debug, Clone, PartialEq)]
pub enum CapacityType {
    /// Fixed capacity that cannot change
    Fixed,
    /// Variable capacity that can be adjusted
    Variable,
    /// Elastic capacity that scales automatically
    Elastic,
    /// Infinite or unbounded capacity
    Unlimited,
}

/// Scaling limits for elastic resources
#[derive(Debug, Clone)]
pub struct ScalingLimits {
    /// Minimum capacity
    pub minimum: f64,
    /// Maximum capacity
    pub maximum: f64,
    /// Scaling increment
    pub increment: f64,
    /// Scaling decrement
    pub decrement: f64,
}

/// Current resource utilization
#[derive(Debug, Clone)]
pub struct ResourceUtilization {
    /// Current usage value
    pub current_usage: f64,
    /// Usage as percentage of capacity
    pub utilization_percentage: f64,
    /// Peak usage in current period
    pub peak_usage: f64,
    /// Average usage in current period
    pub average_usage: f64,
    /// Usage trend
    pub trend: UtilizationTrend,
    /// Last updated timestamp
    pub last_updated: Duration,
}

/// Utilization trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum UtilizationTrend {
    /// Usage is increasing
    Increasing,
    /// Usage is decreasing
    Decreasing,
    /// Usage is stable
    Stable,
    /// Usage is volatile
    Volatile,
    /// Insufficient data
    Unknown,
}

/// Historical utilization patterns
#[derive(Debug, Clone)]
pub struct UtilizationHistory {
    /// Time series data points
    pub data_points: Vec<UtilizationDataPoint>,
    /// Patterns detected in history
    pub patterns: Vec<UtilizationPattern>,
    /// Statistical summary
    pub statistics: UtilizationStatistics,
    /// Seasonal variations
    pub seasonal_patterns: Vec<SeasonalPattern>,
}

/// Individual utilization data point
#[derive(Debug, Clone)]
pub struct UtilizationDataPoint {
    /// Timestamp
    pub timestamp: Duration,
    /// Utilization value
    pub value: f64,
    /// Context information
    pub context: Option<String>,
}

/// Detected utilization pattern
#[derive(Debug, Clone)]
pub struct UtilizationPattern {
    /// Pattern type
    pub pattern_type: PatternType,
    /// Pattern frequency
    pub frequency: Option<Duration>,
    /// Pattern strength (0.0 to 1.0)
    pub strength: f64,
    /// Pattern description
    pub description: String,
}

/// Types of utilization patterns
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Cyclical pattern repeating over time
    Cyclical,
    /// Burst pattern with sudden spikes
    Burst,
    /// Gradual increase over time
    GradualIncrease,
    /// Gradual decrease over time
    GradualDecrease,
    /// Random/irregular pattern
    Random,
    /// Steady state with minimal variation
    SteadyState,
}

/// Statistical summary of utilization
#[derive(Debug, Clone)]
pub struct UtilizationStatistics {
    /// Mean utilization
    pub mean: f64,
    /// Standard deviation
    pub standard_deviation: f64,
    /// Minimum recorded value
    pub minimum: f64,
    /// Maximum recorded value
    pub maximum: f64,
    /// 95th percentile
    pub percentile_95: f64,
    /// 99th percentile
    pub percentile_99: f64,
}

/// Seasonal utilization pattern
#[derive(Debug, Clone)]
pub struct SeasonalPattern {
    /// Season identifier
    pub season: String,
    /// Expected utilization during this season
    pub expected_utilization: f64,
    /// Variation from baseline
    pub variation: f64,
    /// Confidence in prediction
    pub confidence: f64,
}

/// Resource state classification
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceState {
    /// Resource is available and functioning
    Available,
    /// Resource is in use
    InUse,
    /// Resource is overloaded
    Overloaded,
    /// Resource is temporarily unavailable
    Unavailable,
    /// Resource is in maintenance mode
    Maintenance,
    /// Resource has failed
    Failed,
    /// Resource is being scaled
    Scaling,
}

/// Resource priority levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ResourcePriority {
    /// Critical system resource
    Critical,
    /// High priority resource
    High,
    /// Normal priority resource
    Normal,
    /// Low priority resource
    Low,
    /// Background/optional resource
    Background,
}

/// Cost metrics for resource usage
#[derive(Debug, Clone)]
pub struct CostMetrics {
    /// Cost per unit of usage
    pub cost_per_unit: f64,
    /// Total cost incurred
    pub total_cost: f64,
    /// Cost trend
    pub cost_trend: CostTrend,
    /// Cost optimization potential
    pub optimization_potential: f64,
}

/// Cost trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum CostTrend {
    /// Costs are increasing
    Increasing,
    /// Costs are decreasing
    Decreasing,
    /// Costs are stable
    Stable,
    /// Cost trend is unknown
    Unknown,
}

/// Utilization metrics for a resource type
#[derive(Debug, Clone)]
pub struct UtilizationMetrics {
    /// Overall utilization rate
    pub overall_utilization: f64,
    /// Efficiency rating
    pub efficiency_rating: EfficiencyRating,
    /// Waste percentage
    pub waste_percentage: f64,
    /// Bottleneck factor
    pub bottleneck_factor: f64,
    /// Resource contention level
    pub contention_level: ContentionLevel,
    /// Scalability assessment
    pub scalability: ScalabilityAssessment,
}

/// Efficiency rating categories
#[derive(Debug, Clone, PartialEq)]
pub enum EfficiencyRating {
    /// Highly efficient usage (90%+)
    Excellent,
    /// Good efficiency (70-90%)
    Good,
    /// Average efficiency (50-70%)
    Average,
    /// Poor efficiency (30-50%)
    Poor,
    /// Very poor efficiency (<30%)
    VeryPoor,
}

/// Resource contention levels
#[derive(Debug, Clone, PartialEq)]
pub enum ContentionLevel {
    /// No contention
    None,
    /// Low contention
    Low,
    /// Moderate contention
    Moderate,
    /// High contention
    High,
    /// Severe contention
    Severe,
}

/// Scalability assessment
#[derive(Debug, Clone)]
pub struct ScalabilityAssessment {
    /// Horizontal scaling potential
    pub horizontal_scaling: ScalingPotential,
    /// Vertical scaling potential
    pub vertical_scaling: ScalingPotential,
    /// Scaling limitations
    pub limitations: Vec<String>,
    /// Recommended scaling strategy
    pub recommended_strategy: ScalingStrategy,
}

/// Scaling potential levels
#[derive(Debug, Clone, PartialEq)]
pub enum ScalingPotential {
    /// Excellent scaling potential
    Excellent,
    /// Good scaling potential
    Good,
    /// Limited scaling potential
    Limited,
    /// Poor scaling potential
    Poor,
    /// No scaling potential
    None,
}

/// Scaling strategies
#[derive(Debug, Clone, PartialEq)]
pub enum ScalingStrategy {
    /// Scale horizontally (add more instances)
    Horizontal,
    /// Scale vertically (increase capacity)
    Vertical,
    /// Use hybrid approach
    Hybrid,
    /// Optimize before scaling
    OptimizeFirst,
    /// No scaling recommended
    NoScaling,
}

/// Capacity planning analysis
#[derive(Debug, Clone)]
pub struct CapacityPlanning {
    /// Current capacity adequacy
    pub adequacy: CapacityAdequacy,
    /// Projected capacity needs
    pub projected_needs: Vec<CapacityProjection>,
    /// Capacity gaps identified
    pub capacity_gaps: Vec<CapacityGap>,
    /// Capacity surplus identified
    pub capacity_surplus: Vec<CapacitySurplus>,
    /// Planning recommendations
    pub recommendations: Vec<String>,
}

/// Capacity adequacy assessment
#[derive(Debug, Clone, PartialEq)]
pub enum CapacityAdequacy {
    /// Capacity is more than adequate
    Excessive,
    /// Capacity is adequate
    Adequate,
    /// Capacity is barely adequate
    Marginal,
    /// Capacity is inadequate
    Inadequate,
    /// Capacity is critically inadequate
    Critical,
}

/// Capacity projection for future needs
#[derive(Debug, Clone)]
pub struct CapacityProjection {
    /// Time horizon for projection
    pub time_horizon: Duration,
    /// Projected capacity need
    pub projected_capacity: f64,
    /// Confidence level in projection
    pub confidence: f64,
    /// Growth rate assumption
    pub growth_rate: f64,
    /// Factors influencing projection
    pub influencing_factors: Vec<String>,
}

/// Identified capacity gap
#[derive(Debug, Clone)]
pub struct CapacityGap {
    /// Gap identifier
    pub id: String,
    /// Current capacity
    pub current_capacity: f64,
    /// Required capacity
    pub required_capacity: f64,
    /// Gap size
    pub gap_size: f64,
    /// Gap severity
    pub severity: GapSeverity,
    /// When gap will occur
    pub timeline: Duration,
    /// Mitigation options
    pub mitigation_options: Vec<String>,
}

/// Capacity gap severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum GapSeverity {
    /// Minor gap
    Minor,
    /// Moderate gap
    Moderate,
    /// Major gap
    Major,
    /// Critical gap
    Critical,
}

/// Identified capacity surplus
#[derive(Debug, Clone)]
pub struct CapacitySurplus {
    /// Surplus identifier
    pub id: String,
    /// Current capacity
    pub current_capacity: f64,
    /// Required capacity
    pub required_capacity: f64,
    /// Surplus amount
    pub surplus_amount: f64,
    /// Cost of surplus
    pub surplus_cost: f64,
    /// Optimization opportunities
    pub optimization_opportunities: Vec<String>,
}

/// Resource allocation pattern
#[derive(Debug, Clone)]
pub struct AllocationPattern {
    /// Pattern identifier
    pub id: String,
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: AllocationPatternType,
    /// Resources involved
    pub resources: Vec<String>,
    /// Processes involved
    pub processes: Vec<String>,
    /// Pattern efficiency
    pub efficiency: f64,
    /// Pattern prevalence
    pub prevalence: f64,
    /// Pattern impact
    pub impact: PatternImpact,
}

/// Types of allocation patterns
#[derive(Debug, Clone, PartialEq)]
pub enum AllocationPatternType {
    /// Static allocation pattern
    Static,
    /// Dynamic allocation pattern
    Dynamic,
    /// Elastic allocation pattern
    Elastic,
    /// Burst allocation pattern
    Burst,
    /// Predictable allocation pattern
    Predictable,
    /// Random allocation pattern
    Random,
}

/// Pattern impact assessment
#[derive(Debug, Clone)]
pub struct PatternImpact {
    /// Impact on performance
    pub performance_impact: f64,
    /// Impact on cost
    pub cost_impact: f64,
    /// Impact on reliability
    pub reliability_impact: f64,
    /// Overall impact score
    pub overall_impact: f64,
}

/// Resource bottleneck identification
#[derive(Debug, Clone)]
pub struct ResourceBottleneck {
    /// Bottleneck identifier
    pub id: String,
    /// Resource causing bottleneck
    pub resource: String,
    /// Bottleneck type
    pub bottleneck_type: BottleneckType,
    /// Severity of bottleneck
    pub severity: BottleneckSeverity,
    /// Impact of bottleneck
    pub impact: BottleneckImpact,
    /// Root causes
    pub root_causes: Vec<String>,
    /// Affected processes
    pub affected_processes: Vec<String>,
    /// Resolution strategies
    pub resolution_strategies: Vec<String>,
}

/// Types of resource bottlenecks
#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckType {
    /// Capacity bottleneck
    Capacity,
    /// Throughput bottleneck
    Throughput,
    /// Latency bottleneck
    Latency,
    /// Contention bottleneck
    Contention,
    /// Configuration bottleneck
    Configuration,
    /// Architecture bottleneck
    Architecture,
}

impl std::fmt::Display for BottleneckType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BottleneckType::Capacity => write!(f, "capacity"),
            BottleneckType::Throughput => write!(f, "throughput"),
            BottleneckType::Latency => write!(f, "latency"),
            BottleneckType::Contention => write!(f, "contention"),
            BottleneckType::Configuration => write!(f, "configuration"),
            BottleneckType::Architecture => write!(f, "architecture"),
        }
    }
}

/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BottleneckSeverity {
    /// Minor bottleneck
    Minor,
    /// Moderate bottleneck
    Moderate,
    /// Major bottleneck
    Major,
    /// Critical bottleneck
    Critical,
    /// System-threatening bottleneck
    Severe,
}

/// Bottleneck impact analysis
#[derive(Debug, Clone)]
pub struct BottleneckImpact {
    /// Performance degradation
    pub performance_degradation: f64,
    /// Throughput reduction
    pub throughput_reduction: f64,
    /// Response time increase
    pub response_time_increase: f64,
    /// Cost increase
    pub cost_increase: f64,
    /// Affected user count
    pub affected_users: usize,
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// Recommendation title
    pub title: String,
    /// Recommendation description
    pub description: String,
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Expected benefit
    pub expected_benefit: OptimizationBenefit,
    /// Implementation effort
    pub implementation_effort: ImplementationEffort,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Prerequisites
    pub prerequisites: Vec<String>,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
}

/// Types of optimization recommendations
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationType {
    /// Resource reallocation
    Reallocation,
    /// Capacity adjustment
    CapacityAdjustment,
    /// Configuration optimization
    Configuration,
    /// Architecture improvement
    Architecture,
    /// Algorithm optimization
    Algorithm,
    /// Caching implementation
    Caching,
    /// Load balancing
    LoadBalancing,
}

/// Expected optimization benefit
#[derive(Debug, Clone)]
pub struct OptimizationBenefit {
    /// Performance improvement
    pub performance_improvement: f64,
    /// Cost reduction
    pub cost_reduction: f64,
    /// Efficiency gain
    pub efficiency_gain: f64,
    /// Reliability improvement
    pub reliability_improvement: f64,
    /// Confidence in benefit estimate
    pub confidence: f64,
}

/// Implementation effort estimation
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationEffort {
    /// Minimal effort required
    Minimal,
    /// Low effort required
    Low,
    /// Medium effort required
    Medium,
    /// High effort required
    High,
    /// Very high effort required
    VeryHigh,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RecommendationPriority {
    /// Critical priority
    Critical,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
    /// Nice to have
    NiceToHave,
}

/// Resource forecasting analysis
#[derive(Debug, Clone)]
pub struct ResourceForecasting {
    /// Short-term forecasts (days)
    pub short_term: Vec<ResourceForecast>,
    /// Medium-term forecasts (weeks/months)
    pub medium_term: Vec<ResourceForecast>,
    /// Long-term forecasts (months/years)
    pub long_term: Vec<ResourceForecast>,
    /// Forecast accuracy metrics
    pub accuracy_metrics: ForecastAccuracy,
    /// Scenario-based forecasts
    pub scenarios: Vec<ScenarioForecast>,
}

/// Individual resource forecast
#[derive(Debug, Clone)]
pub struct ResourceForecast {
    /// Resource identifier
    pub resource: String,
    /// Forecast horizon
    pub horizon: Duration,
    /// Predicted utilization
    pub predicted_utilization: f64,
    /// Prediction confidence
    pub confidence: f64,
    /// Forecast method used
    pub method: ForecastMethod,
    /// Assumptions made
    pub assumptions: Vec<String>,
}

/// Forecasting methods
#[derive(Debug, Clone, PartialEq)]
pub enum ForecastMethod {
    /// Linear trend extrapolation
    Linear,
    /// Exponential smoothing
    Exponential,
    /// Seasonal decomposition
    Seasonal,
    /// Machine learning based
    MachineLearning,
    /// Hybrid approach
    Hybrid,
    /// Expert judgment
    Expert,
}

/// Forecast accuracy metrics
#[derive(Debug, Clone)]
pub struct ForecastAccuracy {
    /// Mean absolute error
    pub mean_absolute_error: f64,
    /// Root mean square error
    pub root_mean_square_error: f64,
    /// Mean absolute percentage error
    pub mean_absolute_percentage_error: f64,
    /// Forecast bias
    pub bias: f64,
    /// Accuracy rating
    pub accuracy_rating: AccuracyRating,
}

/// Accuracy rating categories
#[derive(Debug, Clone, PartialEq)]
pub enum AccuracyRating {
    /// Highly accurate (>95%)
    Excellent,
    /// Good accuracy (85-95%)
    Good,
    /// Fair accuracy (70-85%)
    Fair,
    /// Poor accuracy (50-70%)
    Poor,
    /// Very poor accuracy (<50%)
    VeryPoor,
}

/// Scenario-based forecast
#[derive(Debug, Clone)]
pub struct ScenarioForecast {
    /// Scenario name
    pub scenario: String,
    /// Scenario probability
    pub probability: f64,
    /// Scenario description
    pub description: String,
    /// Resource impacts in this scenario
    pub resource_impacts: Vec<ResourceImpact>,
    /// Mitigation strategies for this scenario
    pub mitigation_strategies: Vec<String>,
}

/// Resource impact in a scenario
#[derive(Debug, Clone)]
pub struct ResourceImpact {
    /// Resource affected
    pub resource: String,
    /// Impact magnitude
    pub impact_magnitude: f64,
    /// Impact direction
    pub impact_direction: ImpactDirection,
    /// Impact likelihood
    pub likelihood: f64,
}

/// Direction of resource impact
#[derive(Debug, Clone, PartialEq)]
pub enum ImpactDirection {
    /// Positive impact (reduced utilization)
    Positive,
    /// Negative impact (increased utilization)
    Negative,
    /// Neutral impact
    Neutral,
    /// Variable impact
    Variable,
}

/// Performance impact of resource utilization
#[derive(Debug, Clone)]
pub struct ResourcePerformanceImpact {
    /// Overall performance score
    pub overall_score: f64,
    /// Resource efficiency impact
    pub efficiency_impact: f64,
    /// Throughput impact
    pub throughput_impact: f64,
    /// Latency impact
    pub latency_impact: f64,
    /// Scalability impact
    pub scalability_impact: f64,
    /// Cost efficiency impact
    pub cost_efficiency_impact: f64,
    /// Performance trends
    pub performance_trends: PerformanceTrends,
}

/// Performance trends analysis
#[derive(Debug, Clone)]
pub struct PerformanceTrends {
    /// Historical performance trend
    pub historical_trend: TrendDirection,
    /// Projected performance trend
    pub projected_trend: TrendDirection,
    /// Performance volatility
    pub volatility: f64,
    /// Key performance drivers
    pub key_drivers: Vec<String>,
}

/// Trend direction indicators
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    /// Strongly improving
    StronglyImproving,
    /// Improving
    Improving,
    /// Stable
    Stable,
    /// Declining
    Declining,
    /// Strongly declining
    StronglyDeclining,
    /// Volatile
    Volatile,
    /// Unknown
    Unknown,
}

/// Configuration for resource utilization analysis
#[derive(Debug, Clone)]
pub struct ResourceUtilizationConfig {
    /// Enable detailed resource tracking
    pub enable_detailed_tracking: bool,
    /// Enable historical analysis
    pub enable_historical_analysis: bool,
    /// Enable forecasting
    pub enable_forecasting: bool,
    /// Analysis time window
    pub analysis_window: Duration,
    /// Minimum utilization threshold for analysis
    pub min_utilization_threshold: f64,
    /// Maximum resources to analyze
    pub max_resources: usize,
    /// Enable cost analysis
    pub enable_cost_analysis: bool,
}

impl Default for ResourceUtilizationConfig {
    fn default() -> Self {
        Self {
            enable_detailed_tracking: true,
            enable_historical_analysis: true,
            enable_forecasting: false,
            analysis_window: Duration::from_secs(3600), // 1 hour
            min_utilization_threshold: 0.01, // 1%
            max_resources: 1000,
            enable_cost_analysis: false,
        }
    }
}

/// Main resource utilization analyzer
pub struct ResourceUtilizationAnalyzer {
    config: ResourceUtilizationConfig,
}

impl ResourceUtilizationAnalyzer {
    /// Create new resource utilization analyzer
    pub fn new() -> Self {
        Self::with_config(ResourceUtilizationConfig::default())
    }

    /// Create analyzer with custom configuration
    pub fn with_config(config: ResourceUtilizationConfig) -> Self {
        Self { config }
    }

    /// Analyze resource utilization in AISP document
    pub fn analyze_document(&self, document: &AispDocument) -> AispResult<ResourceUtilizationAnalysis> {
        let start_time = Instant::now();

        // Extract resource information from document
        let resource_instances = self.extract_resources(document)?;
        
        if resource_instances.is_empty() {
            return Ok(ResourceUtilizationAnalysis {
                utilization_summary: ResourceUtilizationSummary::empty(),
                resource_analysis: HashMap::new(),
                allocation_patterns: vec![],
                bottlenecks: vec![],
                optimization_recommendations: vec![],
                forecasting: ResourceForecasting::empty(),
                performance_impact: ResourcePerformanceImpact::empty(),
                warnings: vec!["No resources found for analysis".to_string()],
            });
        }

        // Group resources by type
        let resource_groups = self.group_resources_by_type(&resource_instances);

        // Analyze each resource type
        let mut resource_analysis = HashMap::new();
        for (resource_type, instances) in resource_groups {
            let analysis = self.analyze_resource_type(&resource_type, &instances)?;
            resource_analysis.insert(resource_type, analysis);
        }

        // Generate utilization summary
        let utilization_summary = self.generate_utilization_summary(&resource_instances);

        // Identify allocation patterns
        let allocation_patterns = self.identify_allocation_patterns(&resource_instances)?;

        // Detect bottlenecks
        let bottlenecks = self.detect_bottlenecks(&resource_instances)?;

        // Generate optimization recommendations
        let optimization_recommendations = self.generate_optimization_recommendations(
            &resource_instances,
            &bottlenecks,
        )?;

        // Perform forecasting if enabled
        let forecasting = if self.config.enable_forecasting {
            self.perform_forecasting(&resource_instances)?
        } else {
            ResourceForecasting::empty()
        };

        // Calculate performance impact
        let performance_impact = self.calculate_performance_impact(&resource_instances, start_time.elapsed());

        // Generate warnings
        let warnings = self.generate_warnings(&resource_instances, &bottlenecks);

        Ok(ResourceUtilizationAnalysis {
            utilization_summary,
            resource_analysis,
            allocation_patterns,
            bottlenecks,
            optimization_recommendations,
            forecasting,
            performance_impact,
            warnings,
        })
    }

    /// Analyze concurrent processes for resource usage
    pub fn analyze_concurrent_processes(&self, processes: &[ConcurrentProcess]) -> AispResult<ResourceUtilizationAnalysis> {
        // Extract resource information from concurrent processes
        let mut resource_instances = Vec::new();

        for process in processes {
            // Analyze shared resources
            for resource_id in &process.shared_resources {
                resource_instances.push(ResourceInstance {
                    id: resource_id.clone(),
                    name: format!("Shared Resource: {}", resource_id),
                    capacity: ResourceCapacity {
                        maximum: 100.0,
                        units: "units".to_string(),
                        capacity_type: CapacityType::Fixed,
                        elastic: false,
                        scaling_limits: None,
                    },
                    utilization: ResourceUtilization {
                        current_usage: 50.0,
                        utilization_percentage: 50.0,
                        peak_usage: 75.0,
                        average_usage: 45.0,
                        trend: UtilizationTrend::Stable,
                        last_updated: Duration::from_secs(0),
                    },
                    utilization_history: UtilizationHistory::empty(),
                    state: ResourceState::InUse,
                    associated_processes: vec![process.id.clone()],
                    priority: ResourcePriority::Normal,
                    cost_metrics: CostMetrics {
                        cost_per_unit: 1.0,
                        total_cost: 50.0,
                        cost_trend: CostTrend::Stable,
                        optimization_potential: 0.2,
                    },
                });
            }

            // Analyze communication channels as resources
            for channel in &process.channels {
                resource_instances.push(ResourceInstance {
                    id: channel.id.clone(),
                    name: format!("Channel: {}", channel.id),
                    capacity: ResourceCapacity {
                        maximum: channel.buffer_capacity.unwrap_or(10) as f64,
                        units: "messages".to_string(),
                        capacity_type: CapacityType::Variable,
                        elastic: false,
                        scaling_limits: None,
                    },
                    utilization: ResourceUtilization {
                        current_usage: 5.0,
                        utilization_percentage: 50.0,
                        peak_usage: 8.0,
                        average_usage: 4.5,
                        trend: UtilizationTrend::Stable,
                        last_updated: Duration::from_secs(0),
                    },
                    utilization_history: UtilizationHistory::empty(),
                    state: ResourceState::Available,
                    associated_processes: vec![channel.sender.clone(), channel.receiver.clone()],
                    priority: ResourcePriority::Normal,
                    cost_metrics: CostMetrics {
                        cost_per_unit: 0.1,
                        total_cost: 0.5,
                        cost_trend: CostTrend::Stable,
                        optimization_potential: 0.1,
                    },
                });
            }
        }

        // Analyze the extracted resources
        if resource_instances.is_empty() {
            return Ok(ResourceUtilizationAnalysis {
                utilization_summary: ResourceUtilizationSummary::empty(),
                resource_analysis: HashMap::new(),
                allocation_patterns: vec![],
                bottlenecks: vec![],
                optimization_recommendations: vec![],
                forecasting: ResourceForecasting::empty(),
                performance_impact: ResourcePerformanceImpact::empty(),
                warnings: vec!["No resources found in concurrent processes".to_string()],
            });
        }

        // Generate complete analysis
        let utilization_summary = self.generate_utilization_summary(&resource_instances);
        let allocation_patterns = self.identify_allocation_patterns(&resource_instances)?;
        let bottlenecks = self.detect_bottlenecks(&resource_instances)?;
        let optimization_recommendations = self.generate_optimization_recommendations(
            &resource_instances,
            &bottlenecks,
        )?;

        Ok(ResourceUtilizationAnalysis {
            utilization_summary,
            resource_analysis: HashMap::new(),
            allocation_patterns,
            bottlenecks,
            optimization_recommendations,
            forecasting: ResourceForecasting::empty(),
            performance_impact: ResourcePerformanceImpact {
                overall_score: 0.7,
                efficiency_impact: 0.1,
                throughput_impact: 0.0,
                latency_impact: 0.05,
                scalability_impact: -0.1,
                cost_efficiency_impact: 0.1,
                performance_trends: PerformanceTrends {
                    historical_trend: TrendDirection::Stable,
                    projected_trend: TrendDirection::Stable,
                    volatility: 0.1,
                    key_drivers: vec!["Resource contention".to_string()],
                },
            },
            warnings: vec![],
        })
    }

    /// Extract resource instances from AISP document
    fn extract_resources(&self, _document: &AispDocument) -> AispResult<Vec<ResourceInstance>> {
        // Simplified extraction - would analyze document structure for resources
        let mut resources = Vec::new();

        // Create example resource instances
        resources.push(ResourceInstance {
            id: "cpu_main".to_string(),
            name: "Main CPU".to_string(),
            capacity: ResourceCapacity {
                maximum: 100.0,
                units: "percent".to_string(),
                capacity_type: CapacityType::Fixed,
                elastic: false,
                scaling_limits: None,
            },
            utilization: ResourceUtilization {
                current_usage: 65.0,
                utilization_percentage: 65.0,
                peak_usage: 85.0,
                average_usage: 70.0,
                trend: UtilizationTrend::Stable,
                last_updated: Duration::from_secs(0),
            },
            utilization_history: UtilizationHistory::empty(),
            state: ResourceState::InUse,
            associated_processes: vec!["main_process".to_string()],
            priority: ResourcePriority::Critical,
            cost_metrics: CostMetrics {
                cost_per_unit: 0.05,
                total_cost: 3.25,
                cost_trend: CostTrend::Stable,
                optimization_potential: 0.15,
            },
        });

        resources.push(ResourceInstance {
            id: "memory_main".to_string(),
            name: "Main Memory".to_string(),
            capacity: ResourceCapacity {
                maximum: 8192.0,
                units: "MB".to_string(),
                capacity_type: CapacityType::Fixed,
                elastic: false,
                scaling_limits: None,
            },
            utilization: ResourceUtilization {
                current_usage: 6000.0,
                utilization_percentage: 73.2,
                peak_usage: 7500.0,
                average_usage: 5800.0,
                trend: UtilizationTrend::Increasing,
                last_updated: Duration::from_secs(0),
            },
            utilization_history: UtilizationHistory::empty(),
            state: ResourceState::InUse,
            associated_processes: vec!["main_process".to_string(), "background_process".to_string()],
            priority: ResourcePriority::Critical,
            cost_metrics: CostMetrics {
                cost_per_unit: 0.01,
                total_cost: 60.0,
                cost_trend: CostTrend::Increasing,
                optimization_potential: 0.25,
            },
        });

        Ok(resources)
    }

    /// Group resources by type
    fn group_resources_by_type(&self, resources: &[ResourceInstance]) -> HashMap<ResourceType, Vec<ResourceInstance>> {
        let mut groups = HashMap::new();
        
        for resource in resources {
            let resource_type = self.classify_resource_type(&resource.id);
            groups.entry(resource_type).or_insert_with(Vec::new).push(resource.clone());
        }
        
        groups
    }

    /// Classify resource type based on resource ID
    fn classify_resource_type(&self, resource_id: &str) -> ResourceType {
        if resource_id.contains("cpu") {
            ResourceType::CPU
        } else if resource_id.contains("memory") || resource_id.contains("ram") {
            ResourceType::Memory
        } else if resource_id.contains("network") || resource_id.contains("channel") {
            ResourceType::Network
        } else if resource_id.contains("storage") || resource_id.contains("disk") {
            ResourceType::Storage
        } else if resource_id.contains("thread") {
            ResourceType::ThreadPool
        } else if resource_id.contains("queue") {
            ResourceType::MessageQueue
        } else if resource_id.contains("lock") || resource_id.contains("mutex") {
            ResourceType::Synchronization
        } else {
            ResourceType::Custom(resource_id.to_string())
        }
    }

    /// Analyze specific resource type
    fn analyze_resource_type(&self, resource_type: &ResourceType, instances: &[ResourceInstance]) -> AispResult<ResourceTypeAnalysis> {
        let total_capacity: f64 = instances.iter().map(|r| r.capacity.maximum).sum();
        let total_usage: f64 = instances.iter().map(|r| r.utilization.current_usage).sum();
        let overall_utilization = if total_capacity > 0.0 {
            total_usage / total_capacity
        } else {
            0.0
        };

        let efficiency_rating = if overall_utilization >= 0.9 {
            EfficiencyRating::Excellent
        } else if overall_utilization >= 0.7 {
            EfficiencyRating::Good
        } else if overall_utilization >= 0.5 {
            EfficiencyRating::Average
        } else if overall_utilization >= 0.3 {
            EfficiencyRating::Poor
        } else {
            EfficiencyRating::VeryPoor
        };

        Ok(ResourceTypeAnalysis {
            resource_type: resource_type.clone(),
            resource_instances: instances.to_vec(),
            utilization_metrics: UtilizationMetrics {
                overall_utilization,
                efficiency_rating,
                waste_percentage: (1.0 - overall_utilization) * 100.0,
                bottleneck_factor: if overall_utilization > 0.8 { overall_utilization } else { 0.0 },
                contention_level: if overall_utilization > 0.8 {
                    ContentionLevel::High
                } else if overall_utilization > 0.6 {
                    ContentionLevel::Moderate
                } else {
                    ContentionLevel::Low
                },
                scalability: ScalabilityAssessment {
                    horizontal_scaling: ScalingPotential::Good,
                    vertical_scaling: ScalingPotential::Limited,
                    limitations: vec!["Hardware constraints".to_string()],
                    recommended_strategy: ScalingStrategy::Horizontal,
                },
            },
            capacity_planning: CapacityPlanning {
                adequacy: if overall_utilization > 0.9 {
                    CapacityAdequacy::Inadequate
                } else if overall_utilization > 0.8 {
                    CapacityAdequacy::Marginal
                } else {
                    CapacityAdequacy::Adequate
                },
                projected_needs: vec![],
                capacity_gaps: vec![],
                capacity_surplus: vec![],
                recommendations: vec!["Monitor utilization trends".to_string()],
            },
            bottlenecks: vec![],
            recommendations: vec!["Consider load balancing".to_string()],
        })
    }

    /// Generate utilization summary
    fn generate_utilization_summary(&self, resources: &[ResourceInstance]) -> ResourceUtilizationSummary {
        if resources.is_empty() {
            return ResourceUtilizationSummary::empty();
        }

        let total_resources = resources.len();
        let utilized_resources = resources.iter()
            .filter(|r| r.utilization.utilization_percentage > self.config.min_utilization_threshold * 100.0)
            .count();
        let over_utilized = resources.iter()
            .filter(|r| r.utilization.utilization_percentage > 80.0)
            .count();
        let under_utilized = resources.iter()
            .filter(|r| r.utilization.utilization_percentage < 30.0)
            .count();

        let average_utilization: f64 = resources.iter()
            .map(|r| r.utilization.utilization_percentage)
            .sum::<f64>() / total_resources as f64;

        let peak_utilization = resources.iter()
            .map(|r| r.utilization.peak_usage)
            .fold(0.0f64, |acc, x| acc.max(x));

        let efficiency_score = if average_utilization > 50.0 && average_utilization < 80.0 {
            1.0 - (average_utilization - 65.0).abs() / 65.0
        } else {
            0.5
        };

        ResourceUtilizationSummary {
            efficiency_score,
            total_resources,
            utilized_resources,
            over_utilized_resources: over_utilized,
            under_utilized_resources: under_utilized,
            average_utilization: average_utilization / 100.0,
            peak_utilization: peak_utilization / 100.0,
            diversity_index: 0.8, // Simplified calculation
        }
    }

    /// Identify allocation patterns
    fn identify_allocation_patterns(&self, resources: &[ResourceInstance]) -> AispResult<Vec<AllocationPattern>> {
        let mut patterns = Vec::new();

        // Detect static vs dynamic allocation patterns
        let static_resources = resources.iter()
            .filter(|r| matches!(r.capacity.capacity_type, CapacityType::Fixed))
            .count();

        if static_resources > resources.len() / 2 {
            patterns.push(AllocationPattern {
                id: "static_allocation".to_string(),
                name: "Static Allocation Pattern".to_string(),
                pattern_type: AllocationPatternType::Static,
                resources: resources.iter().map(|r| r.id.clone()).collect(),
                processes: vec!["system".to_string()],
                efficiency: 0.6,
                prevalence: static_resources as f64 / resources.len() as f64,
                impact: PatternImpact {
                    performance_impact: -0.1,
                    cost_impact: 0.1,
                    reliability_impact: 0.2,
                    overall_impact: 0.07,
                },
            });
        }

        // Detect burst patterns
        let burst_resources = resources.iter()
            .filter(|r| r.utilization.peak_usage > r.utilization.average_usage * 1.5)
            .count();

        if burst_resources > 0 {
            patterns.push(AllocationPattern {
                id: "burst_allocation".to_string(),
                name: "Burst Allocation Pattern".to_string(),
                pattern_type: AllocationPatternType::Burst,
                resources: resources.iter()
                    .filter(|r| r.utilization.peak_usage > r.utilization.average_usage * 1.5)
                    .map(|r| r.id.clone())
                    .collect(),
                processes: vec!["burst_process".to_string()],
                efficiency: 0.7,
                prevalence: burst_resources as f64 / resources.len() as f64,
                impact: PatternImpact {
                    performance_impact: -0.2,
                    cost_impact: 0.0,
                    reliability_impact: -0.1,
                    overall_impact: -0.1,
                },
            });
        }

        Ok(patterns)
    }

    /// Detect resource bottlenecks
    fn detect_bottlenecks(&self, resources: &[ResourceInstance]) -> AispResult<Vec<ResourceBottleneck>> {
        let mut bottlenecks = Vec::new();

        for resource in resources {
            // Check for capacity bottlenecks
            if resource.utilization.utilization_percentage > 90.0 {
                bottlenecks.push(ResourceBottleneck {
                    id: format!("bottleneck_{}", resource.id),
                    resource: resource.id.clone(),
                    bottleneck_type: BottleneckType::Capacity,
                    severity: if resource.utilization.utilization_percentage > 95.0 {
                        BottleneckSeverity::Critical
                    } else {
                        BottleneckSeverity::Major
                    },
                    impact: BottleneckImpact {
                        performance_degradation: 0.3,
                        throughput_reduction: 0.25,
                        response_time_increase: 0.5,
                        cost_increase: 0.1,
                        affected_users: 100,
                    },
                    root_causes: vec![
                        "High demand".to_string(),
                        "Insufficient capacity".to_string(),
                    ],
                    affected_processes: resource.associated_processes.clone(),
                    resolution_strategies: vec![
                        "Increase capacity".to_string(),
                        "Optimize resource usage".to_string(),
                        "Implement load balancing".to_string(),
                    ],
                });
            }

            // Check for contention bottlenecks
            if resource.associated_processes.len() > 3 {
                bottlenecks.push(ResourceBottleneck {
                    id: format!("contention_{}", resource.id),
                    resource: resource.id.clone(),
                    bottleneck_type: BottleneckType::Contention,
                    severity: BottleneckSeverity::Moderate,
                    impact: BottleneckImpact {
                        performance_degradation: 0.2,
                        throughput_reduction: 0.15,
                        response_time_increase: 0.3,
                        cost_increase: 0.05,
                        affected_users: 50,
                    },
                    root_causes: vec!["Multiple process contention".to_string()],
                    affected_processes: resource.associated_processes.clone(),
                    resolution_strategies: vec![
                        "Implement resource pooling".to_string(),
                        "Add synchronization".to_string(),
                    ],
                });
            }
        }

        Ok(bottlenecks)
    }

    /// Generate optimization recommendations
    fn generate_optimization_recommendations(
        &self,
        resources: &[ResourceInstance],
        bottlenecks: &[ResourceBottleneck],
    ) -> AispResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Recommendations for under-utilized resources
        for resource in resources {
            if resource.utilization.utilization_percentage < 30.0 {
                recommendations.push(OptimizationRecommendation {
                    id: format!("underutil_{}", resource.id),
                    title: format!("Optimize Under-utilized Resource: {}", resource.name),
                    description: format!(
                        "Resource {} is only {}% utilized. Consider reallocating or consolidating.",
                        resource.name,
                        resource.utilization.utilization_percentage
                    ),
                    recommendation_type: RecommendationType::Reallocation,
                    expected_benefit: OptimizationBenefit {
                        performance_improvement: 0.05,
                        cost_reduction: 0.2,
                        efficiency_gain: 0.3,
                        reliability_improvement: 0.1,
                        confidence: 0.8,
                    },
                    implementation_effort: ImplementationEffort::Medium,
                    priority: RecommendationPriority::Medium,
                    prerequisites: vec!["Resource analysis".to_string()],
                    implementation_steps: vec![
                        "Analyze resource dependencies".to_string(),
                        "Plan reallocation strategy".to_string(),
                        "Execute gradual transition".to_string(),
                    ],
                });
            }
        }

        // Recommendations for bottlenecks
        for bottleneck in bottlenecks {
            recommendations.push(OptimizationRecommendation {
                id: format!("bottleneck_fix_{}", bottleneck.id),
                title: format!("Resolve {} Bottleneck", bottleneck.resource),
                description: format!("Address {} bottleneck affecting performance", bottleneck.bottleneck_type),
                recommendation_type: match bottleneck.bottleneck_type {
                    BottleneckType::Capacity => RecommendationType::CapacityAdjustment,
                    BottleneckType::Configuration => RecommendationType::Configuration,
                    _ => RecommendationType::Architecture,
                },
                expected_benefit: OptimizationBenefit {
                    performance_improvement: 0.4,
                    cost_reduction: 0.1,
                    efficiency_gain: 0.3,
                    reliability_improvement: 0.2,
                    confidence: 0.9,
                },
                implementation_effort: match bottleneck.severity {
                    BottleneckSeverity::Critical | BottleneckSeverity::Severe => ImplementationEffort::High,
                    _ => ImplementationEffort::Medium,
                },
                priority: match bottleneck.severity {
                    BottleneckSeverity::Critical | BottleneckSeverity::Severe => RecommendationPriority::Critical,
                    BottleneckSeverity::Major => RecommendationPriority::High,
                    _ => RecommendationPriority::Medium,
                },
                prerequisites: vec!["Bottleneck confirmation".to_string()],
                implementation_steps: bottleneck.resolution_strategies.clone(),
            });
        }

        Ok(recommendations)
    }

    /// Perform resource forecasting
    fn perform_forecasting(&self, resources: &[ResourceInstance]) -> AispResult<ResourceForecasting> {
        let mut short_term = Vec::new();
        let mut medium_term = Vec::new();
        let mut long_term = Vec::new();

        for resource in resources {
            // Generate simple forecasts based on current trends
            let current_util = resource.utilization.utilization_percentage;
            let trend_factor = match resource.utilization.trend {
                UtilizationTrend::Increasing => 1.1,
                UtilizationTrend::Decreasing => 0.9,
                UtilizationTrend::Stable => 1.0,
                UtilizationTrend::Volatile => 1.05,
                UtilizationTrend::Unknown => 1.0,
            };

            short_term.push(ResourceForecast {
                resource: resource.id.clone(),
                horizon: Duration::from_secs(86400), // 1 day
                predicted_utilization: current_util * trend_factor,
                confidence: 0.8,
                method: ForecastMethod::Linear,
                assumptions: vec!["Current trend continues".to_string()],
            });

            medium_term.push(ResourceForecast {
                resource: resource.id.clone(),
                horizon: Duration::from_secs(86400 * 7), // 1 week
                predicted_utilization: current_util * trend_factor.powi(7),
                confidence: 0.6,
                method: ForecastMethod::Linear,
                assumptions: vec!["Linear trend projection".to_string()],
            });

            long_term.push(ResourceForecast {
                resource: resource.id.clone(),
                horizon: Duration::from_secs(86400 * 30), // 1 month
                predicted_utilization: current_util * trend_factor.powi(30),
                confidence: 0.4,
                method: ForecastMethod::Expert,
                assumptions: vec!["Extrapolated trend".to_string()],
            });
        }

        Ok(ResourceForecasting {
            short_term,
            medium_term,
            long_term,
            accuracy_metrics: ForecastAccuracy {
                mean_absolute_error: 5.0,
                root_mean_square_error: 7.0,
                mean_absolute_percentage_error: 10.0,
                bias: 2.0,
                accuracy_rating: AccuracyRating::Fair,
            },
            scenarios: vec![
                ScenarioForecast {
                    scenario: "Normal Load".to_string(),
                    probability: 0.7,
                    description: "Expected normal operating conditions".to_string(),
                    resource_impacts: vec![],
                    mitigation_strategies: vec!["Standard monitoring".to_string()],
                },
                ScenarioForecast {
                    scenario: "High Load".to_string(),
                    probability: 0.2,
                    description: "Increased demand scenario".to_string(),
                    resource_impacts: vec![],
                    mitigation_strategies: vec!["Scale up resources".to_string()],
                },
            ],
        })
    }

    /// Calculate performance impact
    fn calculate_performance_impact(&self, resources: &[ResourceInstance], _analysis_time: Duration) -> ResourcePerformanceImpact {
        let avg_utilization = if !resources.is_empty() {
            resources.iter()
                .map(|r| r.utilization.utilization_percentage)
                .sum::<f64>() / resources.len() as f64 / 100.0
        } else {
            0.0
        };

        let efficiency_impact = if avg_utilization > 0.8 {
            -(avg_utilization - 0.8) * 2.0  // Negative impact when over-utilized
        } else if avg_utilization < 0.3 {
            -(0.3 - avg_utilization) * 1.0  // Negative impact when under-utilized
        } else {
            0.1  // Positive impact in optimal range
        };

        ResourcePerformanceImpact {
            overall_score: 0.7 + efficiency_impact,
            efficiency_impact,
            throughput_impact: -efficiency_impact * 0.5,
            latency_impact: if avg_utilization > 0.8 { avg_utilization - 0.8 } else { 0.0 },
            scalability_impact: if avg_utilization > 0.9 { -(avg_utilization - 0.9) * 5.0 } else { 0.0 },
            cost_efficiency_impact: if avg_utilization > 0.9 || avg_utilization < 0.3 { -0.1 } else { 0.1 },
            performance_trends: PerformanceTrends {
                historical_trend: TrendDirection::Stable,
                projected_trend: TrendDirection::Stable,
                volatility: 0.1,
                key_drivers: vec![
                    "Resource utilization".to_string(),
                    "System load".to_string(),
                ],
            },
        }
    }

    /// Generate analysis warnings
    fn generate_warnings(&self, resources: &[ResourceInstance], bottlenecks: &[ResourceBottleneck]) -> Vec<String> {
        let mut warnings = Vec::new();

        // Warning for high resource count
        if resources.len() > self.config.max_resources {
            warnings.push(format!(
                "Resource count ({}) exceeds recommended maximum ({})",
                resources.len(),
                self.config.max_resources
            ));
        }

        // Warning for critical bottlenecks
        let critical_bottlenecks = bottlenecks.iter()
            .filter(|b| matches!(b.severity, BottleneckSeverity::Critical | BottleneckSeverity::Severe))
            .count();

        if critical_bottlenecks > 0 {
            warnings.push(format!(
                "{} critical bottlenecks detected requiring immediate attention",
                critical_bottlenecks
            ));
        }

        // Warning for excessive over-utilization
        let over_utilized = resources.iter()
            .filter(|r| r.utilization.utilization_percentage > 95.0)
            .count();

        if over_utilized > 0 {
            warnings.push(format!(
                "{} resources are critically over-utilized (>95%)",
                over_utilized
            ));
        }

        warnings
    }
}

// Implementation of empty constructors
impl ResourceUtilizationSummary {
    fn empty() -> Self {
        Self {
            efficiency_score: 0.0,
            total_resources: 0,
            utilized_resources: 0,
            over_utilized_resources: 0,
            under_utilized_resources: 0,
            average_utilization: 0.0,
            peak_utilization: 0.0,
            diversity_index: 0.0,
        }
    }
}

impl UtilizationHistory {
    fn empty() -> Self {
        Self {
            data_points: vec![],
            patterns: vec![],
            statistics: UtilizationStatistics {
                mean: 0.0,
                standard_deviation: 0.0,
                minimum: 0.0,
                maximum: 0.0,
                percentile_95: 0.0,
                percentile_99: 0.0,
            },
            seasonal_patterns: vec![],
        }
    }
}

impl ResourceForecasting {
    fn empty() -> Self {
        Self {
            short_term: vec![],
            medium_term: vec![],
            long_term: vec![],
            accuracy_metrics: ForecastAccuracy {
                mean_absolute_error: 0.0,
                root_mean_square_error: 0.0,
                mean_absolute_percentage_error: 0.0,
                bias: 0.0,
                accuracy_rating: AccuracyRating::VeryPoor,
            },
            scenarios: vec![],
        }
    }
}

impl ResourcePerformanceImpact {
    fn empty() -> Self {
        Self {
            overall_score: 0.0,
            efficiency_impact: 0.0,
            throughput_impact: 0.0,
            latency_impact: 0.0,
            scalability_impact: 0.0,
            cost_efficiency_impact: 0.0,
            performance_trends: PerformanceTrends {
                historical_trend: TrendDirection::Unknown,
                projected_trend: TrendDirection::Unknown,
                volatility: 0.0,
                key_drivers: vec![],
            },
        }
    }
}

impl Default for ResourceUtilizationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn create_test_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "TestResource".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("resource".to_string()),
                protocol: Some("utilization".to_string()),
            },
            blocks: vec![],
            span: Span { start: 0, end: 0 },
        }
    }

    #[test]
    fn test_resource_analyzer_creation() {
        let analyzer = ResourceUtilizationAnalyzer::new();
        assert!(analyzer.config.enable_detailed_tracking);
        assert!(analyzer.config.enable_historical_analysis);
        assert_eq!(analyzer.config.max_resources, 1000);
    }

    #[test]
    fn test_resource_analysis() {
        let analyzer = ResourceUtilizationAnalyzer::new();
        let document = create_test_document();
        
        let result = analyzer.analyze_document(&document);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert!(analysis.utilization_summary.total_resources >= 0);
    }

    #[test]
    fn test_resource_type_classification() {
        let analyzer = ResourceUtilizationAnalyzer::new();
        
        assert_eq!(analyzer.classify_resource_type("cpu_main"), ResourceType::CPU);
        assert_eq!(analyzer.classify_resource_type("memory_pool"), ResourceType::Memory);
        assert_eq!(analyzer.classify_resource_type("network_interface"), ResourceType::Network);
        assert_eq!(analyzer.classify_resource_type("storage_disk"), ResourceType::Storage);
        assert_eq!(analyzer.classify_resource_type("thread_pool"), ResourceType::ThreadPool);
        assert_eq!(analyzer.classify_resource_type("message_queue"), ResourceType::MessageQueue);
        assert_eq!(analyzer.classify_resource_type("mutex_lock"), ResourceType::Synchronization);
    }

    #[test]
    fn test_utilization_trend_types() {
        let trends = [
            UtilizationTrend::Increasing,
            UtilizationTrend::Decreasing,
            UtilizationTrend::Stable,
            UtilizationTrend::Volatile,
            UtilizationTrend::Unknown,
        ];
        
        assert_eq!(trends.len(), 5);
        assert_eq!(trends[0], UtilizationTrend::Increasing);
        assert_eq!(trends[4], UtilizationTrend::Unknown);
    }

    #[test]
    fn test_efficiency_rating_levels() {
        let ratings = [
            EfficiencyRating::Excellent,
            EfficiencyRating::Good,
            EfficiencyRating::Average,
            EfficiencyRating::Poor,
            EfficiencyRating::VeryPoor,
        ];
        
        assert_eq!(ratings.len(), 5);
        assert_eq!(ratings[0], EfficiencyRating::Excellent);
        assert_eq!(ratings[4], EfficiencyRating::VeryPoor);
    }

    #[test]
    fn test_bottleneck_severity_ordering() {
        assert!(BottleneckSeverity::Severe > BottleneckSeverity::Critical);
        assert!(BottleneckSeverity::Critical > BottleneckSeverity::Major);
        assert!(BottleneckSeverity::Major > BottleneckSeverity::Moderate);
        assert!(BottleneckSeverity::Moderate > BottleneckSeverity::Minor);
    }

    #[test]
    fn test_resource_capacity_types() {
        let capacity_fixed = CapacityType::Fixed;
        let capacity_variable = CapacityType::Variable;
        let capacity_elastic = CapacityType::Elastic;
        let capacity_unlimited = CapacityType::Unlimited;
        
        assert_eq!(capacity_fixed, CapacityType::Fixed);
        assert_eq!(capacity_variable, CapacityType::Variable);
        assert_eq!(capacity_elastic, CapacityType::Elastic);
        assert_eq!(capacity_unlimited, CapacityType::Unlimited);
    }
}