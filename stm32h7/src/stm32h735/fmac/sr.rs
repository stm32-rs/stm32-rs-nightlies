///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `YEMPTY` reader - Y buffer empty flag
pub type YEMPTY_R = crate::BitReader;
///Field `YEMPTY` writer - Y buffer empty flag
pub type YEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X1FULL` reader - X1 buffer full flag
pub type X1FULL_R = crate::BitReader;
///Field `X1FULL` writer - X1 buffer full flag
pub type X1FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVFL` reader - Overflow error flag
pub type OVFL_R = crate::BitReader;
///Field `OVFL` writer - Overflow error flag
pub type OVFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNFL` reader - Underflow error flag
pub type UNFL_R = crate::BitReader;
///Field `UNFL` writer - Underflow error flag
pub type UNFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAT` reader - Saturation error flag
pub type SAT_R = crate::BitReader;
///Field `SAT` writer - Saturation error flag
pub type SAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Y buffer empty flag
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - X1 buffer full flag
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Overflow error flag
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Underflow error flag
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Saturation error flag
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("sat", &self.sat())
            .field("unfl", &self.unfl())
            .field("ovfl", &self.ovfl())
            .field("x1full", &self.x1full())
            .field("yempty", &self.yempty())
            .finish()
    }
}
impl W {
    ///Bit 0 - Y buffer empty flag
    #[inline(always)]
    pub fn yempty(&mut self) -> YEMPTY_W<SRrs> {
        YEMPTY_W::new(self, 0)
    }
    ///Bit 1 - X1 buffer full flag
    #[inline(always)]
    pub fn x1full(&mut self) -> X1FULL_W<SRrs> {
        X1FULL_W::new(self, 1)
    }
    ///Bit 8 - Overflow error flag
    #[inline(always)]
    pub fn ovfl(&mut self) -> OVFL_W<SRrs> {
        OVFL_W::new(self, 8)
    }
    ///Bit 9 - Underflow error flag
    #[inline(always)]
    pub fn unfl(&mut self) -> UNFL_W<SRrs> {
        UNFL_W::new(self, 9)
    }
    ///Bit 10 - Saturation error flag
    #[inline(always)]
    pub fn sat(&mut self) -> SAT_W<SRrs> {
        SAT_W::new(self, 10)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H73x.html#FMAC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
