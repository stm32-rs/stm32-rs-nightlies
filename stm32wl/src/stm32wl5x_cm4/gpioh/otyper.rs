///Register `OTYPER` reader
pub type R = crate::R<OTYPERrs>;
///Register `OTYPER` writer
pub type W = crate::W<OTYPERrs>;
/**Port x configuration bits (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_TYPE {
    ///0: Output push-pull (reset state)
    PushPull = 0,
    ///1: Output open-drain
    OpenDrain = 1,
}
impl From<OUTPUT_TYPE> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_TYPE) -> Self {
        variant as u8 != 0
    }
}
///Field `OT3` reader - Port x configuration bits (y = 0..15)
pub type OT3_R = crate::BitReader<OUTPUT_TYPE>;
impl OT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTPUT_TYPE {
        match self.bits {
            false => OUTPUT_TYPE::PushPull,
            true => OUTPUT_TYPE::OpenDrain,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OUTPUT_TYPE::PushPull
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OUTPUT_TYPE::OpenDrain
    }
}
///Field `OT3` writer - Port x configuration bits (y = 0..15)
pub type OT3_W<'a, REG> = crate::BitWriter<'a, REG, OUTPUT_TYPE>;
impl<'a, REG> OT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_TYPE::PushPull)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_TYPE::OpenDrain)
    }
}
impl R {
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTYPER").field("ot3", &self.ot3()).finish()
    }
}
impl W {
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W<'_, OTYPERrs> {
        OT3_W::new(self, 3)
    }
}
/**GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#GPIOH:OTYPER)*/
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
///`read()` method returns [`otyper::R`](R) reader structure
impl crate::Readable for OTYPERrs {}
///`write(|w| ..)` method takes [`otyper::W`](W) writer structure
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPERrs {}
