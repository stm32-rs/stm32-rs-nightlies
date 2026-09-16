///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
/**Data buffer overrun/underrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_IE {
    ///0: No interrupt generation
    Disabled = 0,
    ///1: An interrupt is generated if either an overrun or an underrun error occurred
    Enabled = 1,
}
impl From<OVR_IE> for bool {
    #[inline(always)]
    fn from(variant: OVR_IE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_IE` reader - Data buffer overrun/underrun interrupt enable
pub type OVR_IE_R = crate::BitReader<OVR_IE>;
impl OVR_IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR_IE {
        match self.bits {
            false => OVR_IE::Disabled,
            true => OVR_IE::Enabled,
        }
    }
    ///No interrupt generation
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_IE::Disabled
    }
    ///An interrupt is generated if either an overrun or an underrun error occurred
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_IE::Enabled
    }
}
///Field `OVR_IE` writer - Data buffer overrun/underrun interrupt enable
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG, OVR_IE>;
impl<'a, REG> OVR_IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt generation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_IE::Disabled)
    }
    ///An interrupt is generated if either an overrun or an underrun error occurred
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_IE::Enabled)
    }
}
impl R {
    ///Bit 1 - Data buffer overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("ovr_ie", &self.ovr_ie())
            .finish()
    }
}
impl W {
    ///Bit 1 - Data buffer overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<'_, IERrs> {
        OVR_IE_W::new(self, 1)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#PSSI:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
