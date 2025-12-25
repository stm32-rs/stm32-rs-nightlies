///Register `ECCNMIR` reader
pub type R = crate::R<ECCNMIRrs>;
///Register `ECCNMIR` writer
pub type W = crate::W<ECCNMIRrs>;
/**NMI behavior setup when a double ECC error occurs on flitf data part

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCNMI_MASK_EN {
    ///0: NMI enabled
    Enabled = 0,
    ///1: NMI disabled
    Disabled = 1,
}
impl From<ECCNMI_MASK_EN> for bool {
    #[inline(always)]
    fn from(variant: ECCNMI_MASK_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCNMI_MASK_EN` reader - NMI behavior setup when a double ECC error occurs on flitf data part
pub type ECCNMI_MASK_EN_R = crate::BitReader<ECCNMI_MASK_EN>;
impl ECCNMI_MASK_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCNMI_MASK_EN {
        match self.bits {
            false => ECCNMI_MASK_EN::Enabled,
            true => ECCNMI_MASK_EN::Disabled,
        }
    }
    ///NMI enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCNMI_MASK_EN::Enabled
    }
    ///NMI disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCNMI_MASK_EN::Disabled
    }
}
///Field `ECCNMI_MASK_EN` writer - NMI behavior setup when a double ECC error occurs on flitf data part
pub type ECCNMI_MASK_EN_W<'a, REG> = crate::BitWriter<'a, REG, ECCNMI_MASK_EN>;
impl<'a, REG> ECCNMI_MASK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NMI enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCNMI_MASK_EN::Enabled)
    }
    ///NMI disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCNMI_MASK_EN::Disabled)
    }
}
impl R {
    ///Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part
    #[inline(always)]
    pub fn eccnmi_mask_en(&self) -> ECCNMI_MASK_EN_R {
        ECCNMI_MASK_EN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCNMIR")
            .field("eccnmi_mask_en", &self.eccnmi_mask_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part
    #[inline(always)]
    pub fn eccnmi_mask_en(&mut self) -> ECCNMI_MASK_EN_W<'_, ECCNMIRrs> {
        ECCNMI_MASK_EN_W::new(self, 0)
    }
}
/**SBS flift ECC NMI mask register

You can [`read`](crate::Reg::read) this register and get [`eccnmir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccnmir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:ECCNMIR)*/
pub struct ECCNMIRrs;
impl crate::RegisterSpec for ECCNMIRrs {
    type Ux = u32;
}
///`read()` method returns [`eccnmir::R`](R) reader structure
impl crate::Readable for ECCNMIRrs {}
///`write(|w| ..)` method takes [`eccnmir::W`](W) writer structure
impl crate::Writable for ECCNMIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCNMIR to value 0
impl crate::Resettable for ECCNMIRrs {}
