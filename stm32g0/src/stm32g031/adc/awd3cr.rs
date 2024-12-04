///Register `AWD3CR` reader
pub type R = crate::R<AWD3CRrs>;
///Register `AWD3CR` writer
pub type W = crate::W<AWD3CRrs>;
/**ADC analog watchdog 3 monitored channel selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AWD3CH {
    ///0: ADC analog channel-x is not monitored by AWD3
    NotMonitored = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    Monitored = 1,
}
impl From<AWD3CH> for u32 {
    #[inline(always)]
    fn from(variant: AWD3CH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWD3CH {
    type Ux = u32;
}
impl crate::IsEnum for AWD3CH {}
///Field `AWD3CH` reader - ADC analog watchdog 3 monitored channel selection
pub type AWD3CH_R = crate::FieldReader<AWD3CH>;
impl AWD3CH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWD3CH> {
        match self.bits {
            0 => Some(AWD3CH::NotMonitored),
            1 => Some(AWD3CH::Monitored),
            _ => None,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD3CH::NotMonitored
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD3CH::Monitored
    }
}
///Field `AWD3CH` writer - ADC analog watchdog 3 monitored channel selection
pub type AWD3CH_W<'a, REG> = crate::FieldWriter<'a, REG, 19, AWD3CH>;
impl<'a, REG> AWD3CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH::NotMonitored)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH::Monitored)
    }
}
impl R {
    ///Bits 0:18 - ADC analog watchdog 3 monitored channel selection
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new(self.bits & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3CR")
            .field("awd3ch", &self.awd3ch())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - ADC analog watchdog 3 monitored channel selection
    #[inline(always)]
    pub fn awd3ch(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 0)
    }
}
/**ADC analog watchdog 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#ADC:AWD3CR)*/
pub struct AWD3CRrs;
impl crate::RegisterSpec for AWD3CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd3cr::R`](R) reader structure
impl crate::Readable for AWD3CRrs {}
///`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure
impl crate::Writable for AWD3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWD3CR to value 0
impl crate::Resettable for AWD3CRrs {
    const RESET_VALUE: u32 = 0;
}
