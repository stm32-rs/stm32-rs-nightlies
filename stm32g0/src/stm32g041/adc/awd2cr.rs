///Register `AWD2CR` reader
pub type R = crate::R<AWD2CRrs>;
///Register `AWD2CR` writer
pub type W = crate::W<AWD2CRrs>;
/**ADC analog watchdog 2 monitored channel selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AWD2CH {
    ///0: ADC analog channel-x is not monitored by AWD2
    NotMonitored = 0,
    ///1: ADC analog channel-x is monitored by AWD2
    Monitored = 1,
}
impl From<AWD2CH> for u32 {
    #[inline(always)]
    fn from(variant: AWD2CH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWD2CH {
    type Ux = u32;
}
impl crate::IsEnum for AWD2CH {}
///Field `AWD2CH` reader - ADC analog watchdog 2 monitored channel selection
pub type AWD2CH_R = crate::FieldReader<AWD2CH>;
impl AWD2CH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWD2CH> {
        match self.bits {
            0 => Some(AWD2CH::NotMonitored),
            1 => Some(AWD2CH::Monitored),
            _ => None,
        }
    }
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD2CH::NotMonitored
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD2CH::Monitored
    }
}
///Field `AWD2CH` writer - ADC analog watchdog 2 monitored channel selection
pub type AWD2CH_W<'a, REG> = crate::FieldWriter<'a, REG, 19, AWD2CH>;
impl<'a, REG> AWD2CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///ADC analog channel-x is not monitored by AWD2
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH::NotMonitored)
    }
    ///ADC analog channel-x is monitored by AWD2
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH::Monitored)
    }
}
impl R {
    ///Bits 0:18 - ADC analog watchdog 2 monitored channel selection
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new(self.bits & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2CR")
            .field("awd2ch", &self.awd2ch())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - ADC analog watchdog 2 monitored channel selection
    #[inline(always)]
    pub fn awd2ch(&mut self) -> AWD2CH_W<AWD2CRrs> {
        AWD2CH_W::new(self, 0)
    }
}
/**ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#ADC:AWD2CR)*/
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2cr::R`](R) reader structure
impl crate::Readable for AWD2CRrs {}
///`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWD2CR to value 0
impl crate::Resettable for AWD2CRrs {
    const RESET_VALUE: u32 = 0;
}
