#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<CFGR3rs>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<CFGR3rs>;
#[doc = "VREFINT enable and scaler control for COMP2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_VREFINT {
    #[doc = "0: VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    Disabled = 0,
    #[doc = "1: VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    Enabled = 1,
}
impl From<EN_VREFINT> for bool {
    #[inline(always)]
    fn from(variant: EN_VREFINT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_VREFINT` reader - VREFINT enable and scaler control for COMP2 enable bit"]
pub type EN_VREFINT_R = crate::BitReader<EN_VREFINT>;
impl EN_VREFINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_VREFINT {
        match self.bits {
            false => EN_VREFINT::Disabled,
            true => EN_VREFINT::Enabled,
        }
    }
    #[doc = "VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_VREFINT::Disabled
    }
    #[doc = "VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_VREFINT::Enabled
    }
}
#[doc = "Field `EN_VREFINT` writer - VREFINT enable and scaler control for COMP2 enable bit"]
pub type EN_VREFINT_W<'a, REG> = crate::BitWriter<'a, REG, EN_VREFINT>;
impl<'a, REG> EN_VREFINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN_VREFINT::Disabled)
    }
    #[doc = "VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN_VREFINT::Enabled)
    }
}
#[doc = "BGAP_ADC connection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_VREF_OUT {
    #[doc = "0: no pad connected"]
    NoConnection = 0,
    #[doc = "1: PB0 connected"]
    Pb0 = 1,
    #[doc = "2: PB1 connected"]
    Pb1 = 2,
    #[doc = "3: PB0 and PB1 connected"]
    Both = 3,
}
impl From<SEL_VREF_OUT> for u8 {
    #[inline(always)]
    fn from(variant: SEL_VREF_OUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEL_VREF_OUT {
    type Ux = u8;
}
#[doc = "Field `SEL_VREF_OUT` reader - BGAP_ADC connection bit"]
pub type SEL_VREF_OUT_R = crate::FieldReader<SEL_VREF_OUT>;
impl SEL_VREF_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEL_VREF_OUT {
        match self.bits {
            0 => SEL_VREF_OUT::NoConnection,
            1 => SEL_VREF_OUT::Pb0,
            2 => SEL_VREF_OUT::Pb1,
            3 => SEL_VREF_OUT::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "no pad connected"]
    #[inline(always)]
    pub fn is_no_connection(&self) -> bool {
        *self == SEL_VREF_OUT::NoConnection
    }
    #[doc = "PB0 connected"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == SEL_VREF_OUT::Pb0
    }
    #[doc = "PB1 connected"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == SEL_VREF_OUT::Pb1
    }
    #[doc = "PB0 and PB1 connected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SEL_VREF_OUT::Both
    }
}
#[doc = "Field `SEL_VREF_OUT` writer - BGAP_ADC connection bit"]
pub type SEL_VREF_OUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SEL_VREF_OUT>;
impl<'a, REG> SEL_VREF_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no pad connected"]
    #[inline(always)]
    pub fn no_connection(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::NoConnection)
    }
    #[doc = "PB0 connected"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::Pb0)
    }
    #[doc = "PB1 connected"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::Pb1)
    }
    #[doc = "PB0 and PB1 connected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::Both)
    }
}
#[doc = "VREFINT reference for ADC enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_VREFINT_ADC {
    #[doc = "0: Disables the buffer used to generate VREFINT reference for the ADC"]
    Disabled = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT reference for the ADC"]
    Enabled = 1,
}
impl From<ENBUF_VREFINT_ADC> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_VREFINT_ADC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBUF_VREFINT_ADC` reader - VREFINT reference for ADC enable bit"]
pub type ENBUF_VREFINT_ADC_R = crate::BitReader<ENBUF_VREFINT_ADC>;
impl ENBUF_VREFINT_ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENBUF_VREFINT_ADC {
        match self.bits {
            false => ENBUF_VREFINT_ADC::Disabled,
            true => ENBUF_VREFINT_ADC::Enabled,
        }
    }
    #[doc = "Disables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_VREFINT_ADC::Disabled
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_VREFINT_ADC::Enabled
    }
}
#[doc = "Field `ENBUF_VREFINT_ADC` writer - VREFINT reference for ADC enable bit"]
pub type ENBUF_VREFINT_ADC_W<'a, REG> = crate::BitWriter<'a, REG, ENBUF_VREFINT_ADC>;
impl<'a, REG> ENBUF_VREFINT_ADC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_VREFINT_ADC::Disabled)
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_VREFINT_ADC::Enabled)
    }
}
#[doc = "Sensor reference for ADC enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_SENSOR_ADC {
    #[doc = "0: Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    Disabled = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    Enabled = 1,
}
impl From<ENBUF_SENSOR_ADC> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_SENSOR_ADC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBUF_SENSOR_ADC` reader - Sensor reference for ADC enable bit"]
pub type ENBUF_SENSOR_ADC_R = crate::BitReader<ENBUF_SENSOR_ADC>;
impl ENBUF_SENSOR_ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENBUF_SENSOR_ADC {
        match self.bits {
            false => ENBUF_SENSOR_ADC::Disabled,
            true => ENBUF_SENSOR_ADC::Enabled,
        }
    }
    #[doc = "Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_SENSOR_ADC::Disabled
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_SENSOR_ADC::Enabled
    }
}
#[doc = "Field `ENBUF_SENSOR_ADC` writer - Sensor reference for ADC enable bit"]
pub type ENBUF_SENSOR_ADC_W<'a, REG> = crate::BitWriter<'a, REG, ENBUF_SENSOR_ADC>;
impl<'a, REG> ENBUF_SENSOR_ADC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_SENSOR_ADC::Disabled)
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_SENSOR_ADC::Enabled)
    }
}
#[doc = "VREFINT reference for COMP2 scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_VREFINT_COMP2 {
    #[doc = "0: Disables the buffer used to generate VREFINT references for COMP2"]
    Disabled = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT references for COMP2"]
    Enabled = 1,
}
impl From<ENBUF_VREFINT_COMP2> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_VREFINT_COMP2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP2` reader - VREFINT reference for COMP2 scaler enable bit"]
pub type ENBUF_VREFINT_COMP2_R = crate::BitReader<ENBUF_VREFINT_COMP2>;
impl ENBUF_VREFINT_COMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENBUF_VREFINT_COMP2 {
        match self.bits {
            false => ENBUF_VREFINT_COMP2::Disabled,
            true => ENBUF_VREFINT_COMP2::Enabled,
        }
    }
    #[doc = "Disables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_VREFINT_COMP2::Disabled
    }
    #[doc = "Enables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_VREFINT_COMP2::Enabled
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP2` writer - VREFINT reference for COMP2 scaler enable bit"]
pub type ENBUF_VREFINT_COMP2_W<'a, REG> = crate::BitWriter<'a, REG, ENBUF_VREFINT_COMP2>;
impl<'a, REG> ENBUF_VREFINT_COMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_VREFINT_COMP2::Disabled)
    }
    #[doc = "Enables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_VREFINT_COMP2::Enabled)
    }
}
#[doc = "VREFINT ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFINT_RDYF {
    #[doc = "0: VREFINT OFF"]
    NotReady = 0,
    #[doc = "1: VREFINT ready"]
    Ready = 1,
}
impl From<VREFINT_RDYF> for bool {
    #[inline(always)]
    fn from(variant: VREFINT_RDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFINT_RDYF` reader - VREFINT ready flag"]
pub type VREFINT_RDYF_R = crate::BitReader<VREFINT_RDYF>;
impl VREFINT_RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFINT_RDYF {
        match self.bits {
            false => VREFINT_RDYF::NotReady,
            true => VREFINT_RDYF::Ready,
        }
    }
    #[doc = "VREFINT OFF"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VREFINT_RDYF::NotReady
    }
    #[doc = "VREFINT ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VREFINT_RDYF::Ready
    }
}
#[doc = "SYSCFG_CFGR3 lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_LOCK {
    #[doc = "0: SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    ReadWrite = 0,
    #[doc = "1: SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    ReadOnly = 1,
}
impl From<REF_LOCK> for bool {
    #[inline(always)]
    fn from(variant: REF_LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REF_LOCK` reader - SYSCFG_CFGR3 lock bit"]
pub type REF_LOCK_R = crate::BitReader<REF_LOCK>;
impl REF_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REF_LOCK {
        match self.bits {
            false => REF_LOCK::ReadWrite,
            true => REF_LOCK::ReadOnly,
        }
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == REF_LOCK::ReadWrite
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == REF_LOCK::ReadOnly
    }
}
#[doc = "Field `REF_LOCK` writer - SYSCFG_CFGR3 lock bit"]
pub type REF_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, REF_LOCK>;
impl<'a, REG> REF_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(REF_LOCK::ReadWrite)
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(REF_LOCK::ReadOnly)
    }
}
impl R {
    #[doc = "Bit 0 - VREFINT enable and scaler control for COMP2 enable bit"]
    #[inline(always)]
    pub fn en_vrefint(&self) -> EN_VREFINT_R {
        EN_VREFINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&self) -> SEL_VREF_OUT_R {
        SEL_VREF_OUT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_adc(&self) -> ENBUF_VREFINT_ADC_R {
        ENBUF_VREFINT_ADC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&self) -> ENBUF_SENSOR_ADC_R {
        ENBUF_SENSOR_ADC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - VREFINT reference for COMP2 scaler enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp2(&self) -> ENBUF_VREFINT_COMP2_R {
        ENBUF_VREFINT_COMP2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 30 - VREFINT ready flag"]
    #[inline(always)]
    pub fn vrefint_rdyf(&self) -> VREFINT_RDYF_R {
        VREFINT_RDYF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SYSCFG_CFGR3 lock bit"]
    #[inline(always)]
    pub fn ref_lock(&self) -> REF_LOCK_R {
        REF_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREFINT enable and scaler control for COMP2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_vrefint(&mut self) -> EN_VREFINT_W<CFGR3rs> {
        EN_VREFINT_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sel_vref_out(&mut self) -> SEL_VREF_OUT_W<CFGR3rs> {
        SEL_VREF_OUT_W::new(self, 4)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbuf_vrefint_adc(&mut self) -> ENBUF_VREFINT_ADC_W<CFGR3rs> {
        ENBUF_VREFINT_ADC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbuf_sensor_adc(&mut self) -> ENBUF_SENSOR_ADC_W<CFGR3rs> {
        ENBUF_SENSOR_ADC_W::new(self, 9)
    }
    #[doc = "Bit 12 - VREFINT reference for COMP2 scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbuf_vrefint_comp2(&mut self) -> ENBUF_VREFINT_COMP2_W<CFGR3rs> {
        ENBUF_VREFINT_COMP2_W::new(self, 12)
    }
    #[doc = "Bit 31 - SYSCFG_CFGR3 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn ref_lock(&mut self) -> REF_LOCK_W<CFGR3rs> {
        REF_LOCK_W::new(self, 31)
    }
}
#[doc = "SYSCFG configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for CFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
