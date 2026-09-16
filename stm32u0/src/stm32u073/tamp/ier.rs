///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `TAMP1IE` reader - Tamper 1 interrupt enable
pub type TAMP1IE_R = crate::BitReader;
///Field `TAMP1IE` writer - Tamper 1 interrupt enable
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2IE` reader - Tamper 2 interrupt enable
pub type TAMP2IE_R = crate::BitReader;
///Field `TAMP2IE` writer - Tamper 2 interrupt enable
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3IE` reader - Tamper 3 interrupt enable
pub type TAMP3IE_R = crate::BitReader;
///Field `TAMP3IE` writer - Tamper 3 interrupt enable
pub type TAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4IE` reader - Tamper 4 interrupt enable
pub type TAMP4IE_R = crate::BitReader;
///Field `TAMP4IE` writer - Tamper 4 interrupt enable
pub type TAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5IE` reader - Tamper 5 interrupt enable
pub type TAMP5IE_R = crate::BitReader;
///Field `TAMP5IE` writer - Tamper 5 interrupt enable
pub type TAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable
pub type ITAMP3IE_R = crate::BitReader;
///Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable
pub type ITAMP4IE_R = crate::BitReader;
///Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable
pub type ITAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable
pub type ITAMP5IE_R = crate::BitReader;
///Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable
pub type ITAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable
pub type ITAMP6IE_R = crate::BitReader;
///Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable
pub type ITAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper 3 interrupt enable
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 4 interrupt enable
    #[inline(always)]
    pub fn tamp4ie(&self) -> TAMP4IE_R {
        TAMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tamper 5 interrupt enable
    #[inline(always)]
    pub fn tamp5ie(&self) -> TAMP5IE_R {
        TAMP5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 18 - Internal tamper 3 interrupt enable
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Internal tamper 4 interrupt enable
    #[inline(always)]
    pub fn itamp4ie(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Internal tamper 5 interrupt enable
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Internal tamper 6 interrupt enable
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
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
            .field("itamp3ie", &self.itamp3ie())
            .field("itamp4ie", &self.itamp4ie())
            .field("itamp5ie", &self.itamp5ie())
            .field("itamp6ie", &self.itamp6ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<'_, IERrs> {
        TAMP1IE_W::new(self, 0)
    }
    ///Bit 1 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<'_, IERrs> {
        TAMP2IE_W::new(self, 1)
    }
    ///Bit 2 - Tamper 3 interrupt enable
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<'_, IERrs> {
        TAMP3IE_W::new(self, 2)
    }
    ///Bit 3 - Tamper 4 interrupt enable
    #[inline(always)]
    pub fn tamp4ie(&mut self) -> TAMP4IE_W<'_, IERrs> {
        TAMP4IE_W::new(self, 3)
    }
    ///Bit 4 - Tamper 5 interrupt enable
    #[inline(always)]
    pub fn tamp5ie(&mut self) -> TAMP5IE_W<'_, IERrs> {
        TAMP5IE_W::new(self, 4)
    }
    ///Bit 18 - Internal tamper 3 interrupt enable
    #[inline(always)]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<'_, IERrs> {
        ITAMP3IE_W::new(self, 18)
    }
    ///Bit 19 - Internal tamper 4 interrupt enable
    #[inline(always)]
    pub fn itamp4ie(&mut self) -> ITAMP4IE_W<'_, IERrs> {
        ITAMP4IE_W::new(self, 19)
    }
    ///Bit 20 - Internal tamper 5 interrupt enable
    #[inline(always)]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<'_, IERrs> {
        ITAMP5IE_W::new(self, 20)
    }
    ///Bit 21 - Internal tamper 6 interrupt enable
    #[inline(always)]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<'_, IERrs> {
        ITAMP6IE_W::new(self, 21)
    }
}
/**TAMP interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TAMP:IER)*/
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
