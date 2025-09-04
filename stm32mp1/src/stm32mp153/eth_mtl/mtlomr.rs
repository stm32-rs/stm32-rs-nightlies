///Register `MTLOMR` reader
pub type R = crate::R<MTLOMRrs>;
///Register `MTLOMR` writer
pub type W = crate::W<MTLOMRrs>;
///Field `DTXSTS` reader - DTXSTS
pub type DTXSTS_R = crate::BitReader;
///Field `DTXSTS` writer - DTXSTS
pub type DTXSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAA` reader - RAA
pub type RAA_R = crate::BitReader;
///Field `RAA` writer - RAA
pub type RAA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCHALG` reader - SCHALG
pub type SCHALG_R = crate::FieldReader;
///Field `SCHALG` writer - SCHALG
pub type SCHALG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CNTPRST` reader - CNTPRST
pub type CNTPRST_R = crate::BitReader;
///Field `CNTPRST` writer - CNTPRST
pub type CNTPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTCLR` reader - CNTCLR
pub type CNTCLR_R = crate::BitReader;
///Field `CNTCLR` writer - CNTCLR
pub type CNTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - DTXSTS
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RAA
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 5:6 - SCHALG
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 8 - CNTPRST
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CNTCLR
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLOMR")
            .field("dtxsts", &self.dtxsts())
            .field("raa", &self.raa())
            .field("schalg", &self.schalg())
            .field("cntprst", &self.cntprst())
            .field("cntclr", &self.cntclr())
            .finish()
    }
}
impl W {
    ///Bit 1 - DTXSTS
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DTXSTS_W<MTLOMRrs> {
        DTXSTS_W::new(self, 1)
    }
    ///Bit 2 - RAA
    #[inline(always)]
    pub fn raa(&mut self) -> RAA_W<MTLOMRrs> {
        RAA_W::new(self, 2)
    }
    ///Bits 5:6 - SCHALG
    #[inline(always)]
    pub fn schalg(&mut self) -> SCHALG_W<MTLOMRrs> {
        SCHALG_W::new(self, 5)
    }
    ///Bit 8 - CNTPRST
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W<MTLOMRrs> {
        CNTPRST_W::new(self, 8)
    }
    ///Bit 9 - CNTCLR
    #[inline(always)]
    pub fn cntclr(&mut self) -> CNTCLR_W<MTLOMRrs> {
        CNTCLR_W::new(self, 9)
    }
}
/**The Operating Mode register establishes the Transmit and Receive operating modes and commands.

You can [`read`](crate::Reg::read) this register and get [`mtlomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLOMR)*/
pub struct MTLOMRrs;
impl crate::RegisterSpec for MTLOMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlomr::R`](R) reader structure
impl crate::Readable for MTLOMRrs {}
///`write(|w| ..)` method takes [`mtlomr::W`](W) writer structure
impl crate::Writable for MTLOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLOMR to value 0
impl crate::Resettable for MTLOMRrs {}
