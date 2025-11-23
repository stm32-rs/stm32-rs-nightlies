///Register `SCR` reader
pub type R = crate::R<SCRrs>;
///Register `SCR` writer
pub type W = crate::W<SCRrs>;
/**CALRAF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRAF {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<CALRAF> for bool {
    #[inline(always)]
    fn from(variant: CALRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `CALRAF` reader - CALRAF
pub type CALRAF_R = crate::BitReader<CALRAF>;
impl CALRAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALRAF> {
        match self.bits {
            true => Some(CALRAF::Clear),
            _ => None,
        }
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CALRAF::Clear
    }
}
///Field `CALRAF` writer - CALRAF
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG, CALRAF>;
impl<'a, REG> CALRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CALRAF::Clear)
    }
}
///Field `CALRBF` reader - CALRBF
pub use CALRAF_R as CALRBF_R;
///Field `CWUTF` reader - CWUTF
pub use CALRAF_R as CWUTF_R;
///Field `CTSF` reader - CTSF
pub use CALRAF_R as CTSF_R;
///Field `CTSOVF` reader - CTSOVF
pub use CALRAF_R as CTSOVF_R;
///Field `CITSF` reader - CITSF
pub use CALRAF_R as CITSF_R;
///Field `CALRBF` writer - CALRBF
pub use CALRAF_W as CALRBF_W;
///Field `CWUTF` writer - CWUTF
pub use CALRAF_W as CWUTF_W;
///Field `CTSF` writer - CTSF
pub use CALRAF_W as CTSF_W;
///Field `CTSOVF` writer - CTSOVF
pub use CALRAF_W as CTSOVF_W;
///Field `CITSF` writer - CITSF
pub use CALRAF_W as CITSF_W;
impl R {
    ///Bit 0 - CALRAF
    #[inline(always)]
    pub fn calraf(&self) -> CALRAF_R {
        CALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    pub fn calrbf(&self) -> CALRBF_R {
        CALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    pub fn cwutf(&self) -> CWUTF_R {
        CWUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    pub fn ctsovf(&self) -> CTSOVF_R {
        CTSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    pub fn citsf(&self) -> CITSF_R {
        CITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("calraf", &self.calraf())
            .field("calrbf", &self.calrbf())
            .field("cwutf", &self.cwutf())
            .field("ctsf", &self.ctsf())
            .field("ctsovf", &self.ctsovf())
            .field("citsf", &self.citsf())
            .finish()
    }
}
impl W {
    ///Bit 0 - CALRAF
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W<'_, SCRrs> {
        CALRAF_W::new(self, 0)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W<'_, SCRrs> {
        CALRBF_W::new(self, 1)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W<'_, SCRrs> {
        CWUTF_W::new(self, 2)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W<'_, SCRrs> {
        CTSF_W::new(self, 3)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W<'_, SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W<'_, SCRrs> {
        CITSF_W::new(self, 5)
    }
}
/**status clear register

You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#RTC:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCRrs {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
