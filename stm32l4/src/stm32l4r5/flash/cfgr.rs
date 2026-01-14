///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**Low voltage enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVEN {
    ///0: Flash low voltage disabled
    Disabled = 0,
    ///1: Flash low voltage enabled
    Enabled = 1,
}
impl From<LVEN> for bool {
    #[inline(always)]
    fn from(variant: LVEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LVEN` reader - Low voltage enable
pub type LVEN_R = crate::BitReader<LVEN>;
impl LVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVEN {
        match self.bits {
            false => LVEN::Disabled,
            true => LVEN::Enabled,
        }
    }
    ///Flash low voltage disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LVEN::Disabled
    }
    ///Flash low voltage enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LVEN::Enabled
    }
}
///Field `LVEN` writer - Low voltage enable
pub type LVEN_W<'a, REG> = crate::BitWriter<'a, REG, LVEN>;
impl<'a, REG> LVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash low voltage disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LVEN::Disabled)
    }
    ///Flash low voltage enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LVEN::Enabled)
    }
}
impl R {
    ///Bit 0 - Low voltage enable
    #[inline(always)]
    pub fn lven(&self) -> LVEN_R {
        LVEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR").field("lven", &self.lven()).finish()
    }
}
impl W {
    ///Bit 0 - Low voltage enable
    #[inline(always)]
    pub fn lven(&mut self) -> LVEN_W<'_, CFGRrs> {
        LVEN_W::new(self, 0)
    }
}
/**Flash configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#FLASH:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
