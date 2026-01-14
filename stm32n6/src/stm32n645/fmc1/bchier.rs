///Register `BCHIER` reader
pub type R = crate::R<BCHIERrs>;
///Register `BCHIER` writer
pub type W = crate::W<BCHIERrs>;
///Field `DUEIE` reader - Decoder Uncorrectable Errors Interrupt enable
pub type DUEIE_R = crate::BitReader;
///Field `DUEIE` writer - Decoder Uncorrectable Errors Interrupt enable
pub type DUEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DERIE` reader - Decoder Error Ready Interrupt enable
pub type DERIE_R = crate::BitReader;
///Field `DERIE` writer - Decoder Error Ready Interrupt enable
pub type DERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEFIE` reader - Decoder Error Found Interrupt enable
pub type DEFIE_R = crate::BitReader;
///Field `DEFIE` writer - Decoder Error Found Interrupt enable
pub type DEFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSRIE` reader - Decoder Syndrome Ready Interrupt enable
pub type DSRIE_R = crate::BitReader;
///Field `DSRIE` writer - Decoder Syndrome Ready Interrupt enable
pub type DSRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPBRIE` reader - Decoder Parity Bits Ready Interrupt enable
pub type EPBRIE_R = crate::BitReader;
///Field `EPBRIE` writer - Decoder Parity Bits Ready Interrupt enable
pub type EPBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Decoder Uncorrectable Errors Interrupt enable
    #[inline(always)]
    pub fn dueie(&self) -> DUEIE_R {
        DUEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Decoder Error Ready Interrupt enable
    #[inline(always)]
    pub fn derie(&self) -> DERIE_R {
        DERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Decoder Error Found Interrupt enable
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Decoder Syndrome Ready Interrupt enable
    #[inline(always)]
    pub fn dsrie(&self) -> DSRIE_R {
        DSRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Decoder Parity Bits Ready Interrupt enable
    #[inline(always)]
    pub fn epbrie(&self) -> EPBRIE_R {
        EPBRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHIER")
            .field("dueie", &self.dueie())
            .field("derie", &self.derie())
            .field("defie", &self.defie())
            .field("dsrie", &self.dsrie())
            .field("epbrie", &self.epbrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Decoder Uncorrectable Errors Interrupt enable
    #[inline(always)]
    pub fn dueie(&mut self) -> DUEIE_W<'_, BCHIERrs> {
        DUEIE_W::new(self, 0)
    }
    ///Bit 1 - Decoder Error Ready Interrupt enable
    #[inline(always)]
    pub fn derie(&mut self) -> DERIE_W<'_, BCHIERrs> {
        DERIE_W::new(self, 1)
    }
    ///Bit 2 - Decoder Error Found Interrupt enable
    #[inline(always)]
    pub fn defie(&mut self) -> DEFIE_W<'_, BCHIERrs> {
        DEFIE_W::new(self, 2)
    }
    ///Bit 3 - Decoder Syndrome Ready Interrupt enable
    #[inline(always)]
    pub fn dsrie(&mut self) -> DSRIE_W<'_, BCHIERrs> {
        DSRIE_W::new(self, 3)
    }
    ///Bit 4 - Decoder Parity Bits Ready Interrupt enable
    #[inline(always)]
    pub fn epbrie(&mut self) -> EPBRIE_W<'_, BCHIERrs> {
        EPBRIE_W::new(self, 4)
    }
}
/**FMC BCH interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`bchier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bchier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:BCHIER)*/
pub struct BCHIERrs;
impl crate::RegisterSpec for BCHIERrs {
    type Ux = u32;
}
///`read()` method returns [`bchier::R`](R) reader structure
impl crate::Readable for BCHIERrs {}
///`write(|w| ..)` method takes [`bchier::W`](W) writer structure
impl crate::Writable for BCHIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCHIER to value 0
impl crate::Resettable for BCHIERrs {}
