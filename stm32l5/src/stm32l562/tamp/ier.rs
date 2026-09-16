///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `TAMP1IE` reader - TAMP1IE
pub type TAMP1IE_R = crate::BitReader;
///Field `TAMP1IE` writer - TAMP1IE
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2IE` reader - TAMP2IE
pub type TAMP2IE_R = crate::BitReader;
///Field `TAMP2IE` writer - TAMP2IE
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3IE` reader - TAMP3IE
pub type TAMP3IE_R = crate::BitReader;
///Field `TAMP3IE` writer - TAMP3IE
pub type TAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4IE` reader - TAMP4IE
pub type TAMP4IE_R = crate::BitReader;
///Field `TAMP4IE` writer - TAMP4IE
pub type TAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5IE` reader - TAMP5IE
pub type TAMP5IE_R = crate::BitReader;
///Field `TAMP5IE` writer - TAMP5IE
pub type TAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6IE` reader - TAMP6IE
pub type TAMP6IE_R = crate::BitReader;
///Field `TAMP6IE` writer - TAMP6IE
pub type TAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP7IE` reader - TAMP7IE
pub type TAMP7IE_R = crate::BitReader;
///Field `TAMP7IE` writer - TAMP7IE
pub type TAMP7IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP8IE` reader - TAMP8IE
pub type TAMP8IE_R = crate::BitReader;
///Field `TAMP8IE` writer - TAMP8IE
pub type TAMP8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP1IE` reader - ITAMP1IE
pub type ITAMP1IE_R = crate::BitReader;
///Field `ITAMP1IE` writer - ITAMP1IE
pub type ITAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP2IE` reader - ITAMP2IE
pub type ITAMP2IE_R = crate::BitReader;
///Field `ITAMP2IE` writer - ITAMP2IE
pub type ITAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3IE` reader - ITAMP3IE
pub type ITAMP3IE_R = crate::BitReader;
///Field `ITAMP3IE` writer - ITAMP3IE
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5IE` reader - ITAMP5IE
pub type ITAMP5IE_R = crate::BitReader;
///Field `ITAMP5IE` writer - ITAMP5IE
pub type ITAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP8IE` reader - ITAMP8IE
pub type ITAMP8IE_R = crate::BitReader;
///Field `ITAMP8IE` writer - ITAMP8IE
pub type ITAMP8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TAMP1IE
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2IE
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3IE
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4IE
    #[inline(always)]
    pub fn tamp4ie(&self) -> TAMP4IE_R {
        TAMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5IE
    #[inline(always)]
    pub fn tamp5ie(&self) -> TAMP5IE_R {
        TAMP5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6IE
    #[inline(always)]
    pub fn tamp6ie(&self) -> TAMP6IE_R {
        TAMP6IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7IE
    #[inline(always)]
    pub fn tamp7ie(&self) -> TAMP7IE_R {
        TAMP7IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8IE
    #[inline(always)]
    pub fn tamp8ie(&self) -> TAMP8IE_R {
        TAMP8IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - ITAMP1IE
    #[inline(always)]
    pub fn itamp1ie(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ITAMP2IE
    #[inline(always)]
    pub fn itamp2ie(&self) -> ITAMP2IE_R {
        ITAMP2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ITAMP3IE
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5IE
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 23 - ITAMP8IE
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("tamp1ie", &self.tamp1ie())
            .field("tamp2ie", &self.tamp2ie())
            .field("tamp3ie", &self.tamp3ie())
            .field("tamp4ie", &self.tamp4ie())
            .field("tamp5ie", &self.tamp5ie())
            .field("tamp6ie", &self.tamp6ie())
            .field("tamp7ie", &self.tamp7ie())
            .field("tamp8ie", &self.tamp8ie())
            .field("itamp1ie", &self.itamp1ie())
            .field("itamp2ie", &self.itamp2ie())
            .field("itamp3ie", &self.itamp3ie())
            .field("itamp5ie", &self.itamp5ie())
            .field("itamp8ie", &self.itamp8ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TAMP1IE
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<'_, IERrs> {
        TAMP1IE_W::new(self, 0)
    }
    ///Bit 1 - TAMP2IE
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<'_, IERrs> {
        TAMP2IE_W::new(self, 1)
    }
    ///Bit 2 - TAMP3IE
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<'_, IERrs> {
        TAMP3IE_W::new(self, 2)
    }
    ///Bit 3 - TAMP4IE
    #[inline(always)]
    pub fn tamp4ie(&mut self) -> TAMP4IE_W<'_, IERrs> {
        TAMP4IE_W::new(self, 3)
    }
    ///Bit 4 - TAMP5IE
    #[inline(always)]
    pub fn tamp5ie(&mut self) -> TAMP5IE_W<'_, IERrs> {
        TAMP5IE_W::new(self, 4)
    }
    ///Bit 5 - TAMP6IE
    #[inline(always)]
    pub fn tamp6ie(&mut self) -> TAMP6IE_W<'_, IERrs> {
        TAMP6IE_W::new(self, 5)
    }
    ///Bit 6 - TAMP7IE
    #[inline(always)]
    pub fn tamp7ie(&mut self) -> TAMP7IE_W<'_, IERrs> {
        TAMP7IE_W::new(self, 6)
    }
    ///Bit 7 - TAMP8IE
    #[inline(always)]
    pub fn tamp8ie(&mut self) -> TAMP8IE_W<'_, IERrs> {
        TAMP8IE_W::new(self, 7)
    }
    ///Bit 16 - ITAMP1IE
    #[inline(always)]
    pub fn itamp1ie(&mut self) -> ITAMP1IE_W<'_, IERrs> {
        ITAMP1IE_W::new(self, 16)
    }
    ///Bit 17 - ITAMP2IE
    #[inline(always)]
    pub fn itamp2ie(&mut self) -> ITAMP2IE_W<'_, IERrs> {
        ITAMP2IE_W::new(self, 17)
    }
    ///Bit 18 - ITAMP3IE
    #[inline(always)]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<'_, IERrs> {
        ITAMP3IE_W::new(self, 18)
    }
    ///Bit 20 - ITAMP5IE
    #[inline(always)]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<'_, IERrs> {
        ITAMP5IE_W::new(self, 20)
    }
    ///Bit 23 - ITAMP8IE
    #[inline(always)]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<'_, IERrs> {
        ITAMP8IE_W::new(self, 23)
    }
}
/**TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TAMP:IER)*/
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
