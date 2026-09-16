///Register `DADDR` reader
pub type R = crate::R<DADDRrs>;
///Register `DADDR` writer
pub type W = crate::W<DADDRrs>;
///Field `ADD` reader - Device address
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Device address
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Enable function

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EF {
    ///0: USB device disabled
    Disabled = 0,
    ///1: USB device enabled
    Enabled = 1,
}
impl From<EF> for bool {
    #[inline(always)]
    fn from(variant: EF) -> Self {
        variant as u8 != 0
    }
}
///Field `EF` reader - Enable function
pub type EF_R = crate::BitReader<EF>;
impl EF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EF {
        match self.bits {
            false => EF::Disabled,
            true => EF::Enabled,
        }
    }
    ///USB device disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EF::Disabled
    }
    ///USB device enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EF::Enabled
    }
}
///Field `EF` writer - Enable function
pub type EF_W<'a, REG> = crate::BitWriter<'a, REG, EF>;
impl<'a, REG> EF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USB device disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EF::Disabled)
    }
    ///USB device enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EF::Enabled)
    }
}
impl R {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DADDR")
            .field("add", &self.add())
            .field("ef", &self.ef())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, DADDRrs> {
        ADD_W::new(self, 0)
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W<'_, DADDRrs> {
        EF_W::new(self, 7)
    }
}
/**device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#USB:DADDR)*/
pub struct DADDRrs;
impl crate::RegisterSpec for DADDRrs {
    type Ux = u32;
}
///`read()` method returns [`daddr::R`](R) reader structure
impl crate::Readable for DADDRrs {}
///`write(|w| ..)` method takes [`daddr::W`](W) writer structure
impl crate::Writable for DADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DADDR to value 0
impl crate::Resettable for DADDRrs {}
