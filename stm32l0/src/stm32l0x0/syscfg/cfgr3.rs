///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
///Field `EN_BGAP` reader - Vref Enable bit
pub type EN_BGAP_R = crate::BitReader;
///Field `EN_BGAP` writer - Vref Enable bit
pub type EN_BGAP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**BGAP_ADC connection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_VREF_OUT {
    ///0: no pad connected
    NoConnection = 0,
    ///1: PB0 connected
    Pb0 = 1,
    ///2: PB1 connected
    Pb1 = 2,
    ///3: PB0 and PB1 connected
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
impl crate::IsEnum for SEL_VREF_OUT {}
///Field `SEL_VREF_OUT` reader - BGAP_ADC connection bit
pub type SEL_VREF_OUT_R = crate::FieldReader<SEL_VREF_OUT>;
impl SEL_VREF_OUT_R {
    ///Get enumerated values variant
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
    ///no pad connected
    #[inline(always)]
    pub fn is_no_connection(&self) -> bool {
        *self == SEL_VREF_OUT::NoConnection
    }
    ///PB0 connected
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == SEL_VREF_OUT::Pb0
    }
    ///PB1 connected
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == SEL_VREF_OUT::Pb1
    }
    ///PB0 and PB1 connected
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SEL_VREF_OUT::Both
    }
}
///Field `SEL_VREF_OUT` writer - BGAP_ADC connection bit
pub type SEL_VREF_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SEL_VREF_OUT, crate::Safe>;
impl<'a, REG> SEL_VREF_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///no pad connected
    #[inline(always)]
    pub fn no_connection(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::NoConnection)
    }
    ///PB0 connected
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::Pb0)
    }
    ///PB1 connected
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::Pb1)
    }
    ///PB0 and PB1 connected
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_VREF_OUT::Both)
    }
}
///Field `ENBUF_BGAP_ADC` reader - VREFINT reference for ADC enable bit
pub type ENBUF_BGAP_ADC_R = crate::BitReader;
///Field `ENBUF_BGAP_ADC` writer - VREFINT reference for ADC enable bit
pub type ENBUF_BGAP_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Sensor reference for ADC enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_SENSOR_ADC {
    ///0: Disables the buffer used to generate VREFINT reference for the temperature sensor
    Disabled = 0,
    ///1: Enables the buffer used to generate VREFINT reference for the temperature sensor
    Enabled = 1,
}
impl From<ENBUF_SENSOR_ADC> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_SENSOR_ADC) -> Self {
        variant as u8 != 0
    }
}
///Field `ENBUF_SENSOR_ADC` reader - Sensor reference for ADC enable bit
pub type ENBUF_SENSOR_ADC_R = crate::BitReader<ENBUF_SENSOR_ADC>;
impl ENBUF_SENSOR_ADC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENBUF_SENSOR_ADC {
        match self.bits {
            false => ENBUF_SENSOR_ADC::Disabled,
            true => ENBUF_SENSOR_ADC::Enabled,
        }
    }
    ///Disables the buffer used to generate VREFINT reference for the temperature sensor
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_SENSOR_ADC::Disabled
    }
    ///Enables the buffer used to generate VREFINT reference for the temperature sensor
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_SENSOR_ADC::Enabled
    }
}
///Field `ENBUF_SENSOR_ADC` writer - Sensor reference for ADC enable bit
pub type ENBUF_SENSOR_ADC_W<'a, REG> = crate::BitWriter<'a, REG, ENBUF_SENSOR_ADC>;
impl<'a, REG> ENBUF_SENSOR_ADC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables the buffer used to generate VREFINT reference for the temperature sensor
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_SENSOR_ADC::Disabled)
    }
    ///Enables the buffer used to generate VREFINT reference for the temperature sensor
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENBUF_SENSOR_ADC::Enabled)
    }
}
///Field `ENBUF_VREFINT_COMP` reader - VREFINT reference for comparator 2 enable bit
pub type ENBUF_VREFINT_COMP_R = crate::BitReader;
///Field `ENBUF_VREFINT_COMP` writer - VREFINT reference for comparator 2 enable bit
pub type ENBUF_VREFINT_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENREF_RC48MHz` reader - VREFINT reference for 48 MHz RC oscillator enable bit
pub type ENREF_RC48MHZ_R = crate::BitReader;
///Field `ENREF_RC48MHz` writer - VREFINT reference for 48 MHz RC oscillator enable bit
pub type ENREF_RC48MHZ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REF_RC48MHz_RDYF` reader - VREFINT for 48 MHz RC oscillator ready flag
pub type REF_RC48MHZ_RDYF_R = crate::BitReader;
///Field `SENSOR_ADC_RDYF` reader - Sensor for ADC ready flag
pub type SENSOR_ADC_RDYF_R = crate::BitReader;
///Field `VREFINT_ADC_RDYF` reader - VREFINT for ADC ready flag
pub type VREFINT_ADC_RDYF_R = crate::BitReader;
///Field `VREFINT_COMP_RDYF` reader - VREFINT for comparator ready flag
pub type VREFINT_COMP_RDYF_R = crate::BitReader;
/**VREFINT ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFINT_RDYF {
    ///0: VREFINT OFF
    NotReady = 0,
    ///1: VREFINT ready
    Ready = 1,
}
impl From<VREFINT_RDYF> for bool {
    #[inline(always)]
    fn from(variant: VREFINT_RDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFINT_RDYF` reader - VREFINT ready flag
pub type VREFINT_RDYF_R = crate::BitReader<VREFINT_RDYF>;
impl VREFINT_RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFINT_RDYF {
        match self.bits {
            false => VREFINT_RDYF::NotReady,
            true => VREFINT_RDYF::Ready,
        }
    }
    ///VREFINT OFF
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VREFINT_RDYF::NotReady
    }
    ///VREFINT ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VREFINT_RDYF::Ready
    }
}
/**REF_CTRL lock bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_LOCK {
    ///0: SYSCFG_CFGR3\[31:0\] bits are read/write
    ReadWrite = 0,
    ///1: SYSCFG_CFGR3\[31:0\] bits are read-only
    ReadOnly = 1,
}
impl From<REF_LOCK> for bool {
    #[inline(always)]
    fn from(variant: REF_LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `REF_LOCK` writer - REF_CTRL lock bit
pub type REF_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, REF_LOCK>;
impl<'a, REG> REF_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SYSCFG_CFGR3\[31:0\] bits are read/write
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(REF_LOCK::ReadWrite)
    }
    ///SYSCFG_CFGR3\[31:0\] bits are read-only
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(REF_LOCK::ReadOnly)
    }
}
impl R {
    ///Bit 0 - Vref Enable bit
    #[inline(always)]
    pub fn en_bgap(&self) -> EN_BGAP_R {
        EN_BGAP_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - BGAP_ADC connection bit
    #[inline(always)]
    pub fn sel_vref_out(&self) -> SEL_VREF_OUT_R {
        SEL_VREF_OUT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - VREFINT reference for ADC enable bit
    #[inline(always)]
    pub fn enbuf_bgap_adc(&self) -> ENBUF_BGAP_ADC_R {
        ENBUF_BGAP_ADC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Sensor reference for ADC enable bit
    #[inline(always)]
    pub fn enbuf_sensor_adc(&self) -> ENBUF_SENSOR_ADC_R {
        ENBUF_SENSOR_ADC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - VREFINT reference for comparator 2 enable bit
    #[inline(always)]
    pub fn enbuf_vrefint_comp(&self) -> ENBUF_VREFINT_COMP_R {
        ENBUF_VREFINT_COMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - VREFINT reference for 48 MHz RC oscillator enable bit
    #[inline(always)]
    pub fn enref_rc48mhz(&self) -> ENREF_RC48MHZ_R {
        ENREF_RC48MHZ_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 26 - VREFINT for 48 MHz RC oscillator ready flag
    #[inline(always)]
    pub fn ref_rc48mhz_rdyf(&self) -> REF_RC48MHZ_RDYF_R {
        REF_RC48MHZ_RDYF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Sensor for ADC ready flag
    #[inline(always)]
    pub fn sensor_adc_rdyf(&self) -> SENSOR_ADC_RDYF_R {
        SENSOR_ADC_RDYF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - VREFINT for ADC ready flag
    #[inline(always)]
    pub fn vrefint_adc_rdyf(&self) -> VREFINT_ADC_RDYF_R {
        VREFINT_ADC_RDYF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - VREFINT for comparator ready flag
    #[inline(always)]
    pub fn vrefint_comp_rdyf(&self) -> VREFINT_COMP_RDYF_R {
        VREFINT_COMP_RDYF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - VREFINT ready flag
    #[inline(always)]
    pub fn vrefint_rdyf(&self) -> VREFINT_RDYF_R {
        VREFINT_RDYF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("vrefint_rdyf", &self.vrefint_rdyf())
            .field("vrefint_comp_rdyf", &self.vrefint_comp_rdyf())
            .field("vrefint_adc_rdyf", &self.vrefint_adc_rdyf())
            .field("sensor_adc_rdyf", &self.sensor_adc_rdyf())
            .field("ref_rc48mhz_rdyf", &self.ref_rc48mhz_rdyf())
            .field("enref_rc48mhz", &self.enref_rc48mhz())
            .field("enbuf_vrefint_comp", &self.enbuf_vrefint_comp())
            .field("enbuf_sensor_adc", &self.enbuf_sensor_adc())
            .field("enbuf_bgap_adc", &self.enbuf_bgap_adc())
            .field("sel_vref_out", &self.sel_vref_out())
            .field("en_bgap", &self.en_bgap())
            .finish()
    }
}
impl W {
    ///Bit 0 - Vref Enable bit
    #[inline(always)]
    pub fn en_bgap(&mut self) -> EN_BGAP_W<'_, CFGR3rs> {
        EN_BGAP_W::new(self, 0)
    }
    ///Bits 4:5 - BGAP_ADC connection bit
    #[inline(always)]
    pub fn sel_vref_out(&mut self) -> SEL_VREF_OUT_W<'_, CFGR3rs> {
        SEL_VREF_OUT_W::new(self, 4)
    }
    ///Bit 8 - VREFINT reference for ADC enable bit
    #[inline(always)]
    pub fn enbuf_bgap_adc(&mut self) -> ENBUF_BGAP_ADC_W<'_, CFGR3rs> {
        ENBUF_BGAP_ADC_W::new(self, 8)
    }
    ///Bit 9 - Sensor reference for ADC enable bit
    #[inline(always)]
    pub fn enbuf_sensor_adc(&mut self) -> ENBUF_SENSOR_ADC_W<'_, CFGR3rs> {
        ENBUF_SENSOR_ADC_W::new(self, 9)
    }
    ///Bit 12 - VREFINT reference for comparator 2 enable bit
    #[inline(always)]
    pub fn enbuf_vrefint_comp(&mut self) -> ENBUF_VREFINT_COMP_W<'_, CFGR3rs> {
        ENBUF_VREFINT_COMP_W::new(self, 12)
    }
    ///Bit 13 - VREFINT reference for 48 MHz RC oscillator enable bit
    #[inline(always)]
    pub fn enref_rc48mhz(&mut self) -> ENREF_RC48MHZ_W<'_, CFGR3rs> {
        ENREF_RC48MHZ_W::new(self, 13)
    }
    ///Bit 31 - REF_CTRL lock bit
    #[inline(always)]
    pub fn ref_lock(&mut self) -> REF_LOCK_W<'_, CFGR3rs> {
        REF_LOCK_W::new(self, 31)
    }
}
/**SYSCFG configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#SYSCFG:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
