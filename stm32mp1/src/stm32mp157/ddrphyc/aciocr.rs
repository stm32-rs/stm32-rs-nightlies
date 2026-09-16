///Register `ACIOCR` reader
pub type R = crate::R<ACIOCRrs>;
///Register `ACIOCR` writer
pub type W = crate::W<ACIOCRrs>;
///Field `ACIOM` reader - ACIOM
pub type ACIOM_R = crate::BitReader;
///Field `ACIOM` writer - ACIOM
pub type ACIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACOE` reader - ACOE
pub type ACOE_R = crate::BitReader;
///Field `ACOE` writer - ACOE
pub type ACOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACODT` reader - ACODT
pub type ACODT_R = crate::BitReader;
///Field `ACODT` writer - ACODT
pub type ACODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACPDD` reader - ACPDD
pub type ACPDD_R = crate::BitReader;
///Field `ACPDD` writer - ACPDD
pub type ACPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACPDR` reader - ACPDR
pub type ACPDR_R = crate::BitReader;
///Field `ACPDR` writer - ACPDR
pub type ACPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKODT` reader - CKODT
pub type CKODT_R = crate::FieldReader;
///Field `CKODT` writer - CKODT
pub type CKODT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CKPDD` reader - CKPDD
pub type CKPDD_R = crate::FieldReader;
///Field `CKPDD` writer - CKPDD
pub type CKPDD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CKPDR` reader - CKPDR
pub type CKPDR_R = crate::FieldReader;
///Field `CKPDR` writer - CKPDR
pub type CKPDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RANKODT` reader - RANKODT
pub type RANKODT_R = crate::BitReader;
///Field `RANKODT` writer - RANKODT
pub type RANKODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPDD` reader - CSPDD
pub type CSPDD_R = crate::BitReader;
///Field `CSPDD` writer - CSPDD
pub type CSPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RANKPDR` reader - RANKPDR
pub type RANKPDR_R = crate::BitReader;
///Field `RANKPDR` writer - RANKPDR
pub type RANKPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTODT` reader - RSTODT
pub type RSTODT_R = crate::BitReader;
///Field `RSTODT` writer - RSTODT
pub type RSTODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPDD` reader - RSTPDD
pub type RSTPDD_R = crate::BitReader;
///Field `RSTPDD` writer - RSTPDD
pub type RSTPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTPDR` reader - RSTPDR
pub type RSTPDR_R = crate::BitReader;
///Field `RSTPDR` writer - RSTPDR
pub type RSTPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTIOM` reader - RSTIOM
pub type RSTIOM_R = crate::BitReader;
///Field `RSTIOM` writer - RSTIOM
pub type RSTIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACSR` reader - ACSR
pub type ACSR_R = crate::FieldReader;
///Field `ACSR` writer - ACSR
pub type ACSR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - ACIOM
    #[inline(always)]
    pub fn aciom(&self) -> ACIOM_R {
        ACIOM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACOE
    #[inline(always)]
    pub fn acoe(&self) -> ACOE_R {
        ACOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACODT
    #[inline(always)]
    pub fn acodt(&self) -> ACODT_R {
        ACODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ACPDD
    #[inline(always)]
    pub fn acpdd(&self) -> ACPDD_R {
        ACPDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ACPDR
    #[inline(always)]
    pub fn acpdr(&self) -> ACPDR_R {
        ACPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - CKODT
    #[inline(always)]
    pub fn ckodt(&self) -> CKODT_R {
        CKODT_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - CKPDD
    #[inline(always)]
    pub fn ckpdd(&self) -> CKPDD_R {
        CKPDD_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - CKPDR
    #[inline(always)]
    pub fn ckpdr(&self) -> CKPDR_R {
        CKPDR_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 14 - RANKODT
    #[inline(always)]
    pub fn rankodt(&self) -> RANKODT_R {
        RANKODT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - CSPDD
    #[inline(always)]
    pub fn cspdd(&self) -> CSPDD_R {
        CSPDD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - RANKPDR
    #[inline(always)]
    pub fn rankpdr(&self) -> RANKPDR_R {
        RANKPDR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 26 - RSTODT
    #[inline(always)]
    pub fn rstodt(&self) -> RSTODT_R {
        RSTODT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - RSTPDD
    #[inline(always)]
    pub fn rstpdd(&self) -> RSTPDD_R {
        RSTPDD_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - RSTPDR
    #[inline(always)]
    pub fn rstpdr(&self) -> RSTPDR_R {
        RSTPDR_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RSTIOM
    #[inline(always)]
    pub fn rstiom(&self) -> RSTIOM_R {
        RSTIOM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - ACSR
    #[inline(always)]
    pub fn acsr(&self) -> ACSR_R {
        ACSR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACIOCR")
            .field("aciom", &self.aciom())
            .field("acoe", &self.acoe())
            .field("acodt", &self.acodt())
            .field("acpdd", &self.acpdd())
            .field("acpdr", &self.acpdr())
            .field("ckodt", &self.ckodt())
            .field("ckpdd", &self.ckpdd())
            .field("ckpdr", &self.ckpdr())
            .field("rankodt", &self.rankodt())
            .field("cspdd", &self.cspdd())
            .field("rankpdr", &self.rankpdr())
            .field("rstodt", &self.rstodt())
            .field("rstpdd", &self.rstpdd())
            .field("rstpdr", &self.rstpdr())
            .field("rstiom", &self.rstiom())
            .field("acsr", &self.acsr())
            .finish()
    }
}
impl W {
    ///Bit 0 - ACIOM
    #[inline(always)]
    pub fn aciom(&mut self) -> ACIOM_W<'_, ACIOCRrs> {
        ACIOM_W::new(self, 0)
    }
    ///Bit 1 - ACOE
    #[inline(always)]
    pub fn acoe(&mut self) -> ACOE_W<'_, ACIOCRrs> {
        ACOE_W::new(self, 1)
    }
    ///Bit 2 - ACODT
    #[inline(always)]
    pub fn acodt(&mut self) -> ACODT_W<'_, ACIOCRrs> {
        ACODT_W::new(self, 2)
    }
    ///Bit 3 - ACPDD
    #[inline(always)]
    pub fn acpdd(&mut self) -> ACPDD_W<'_, ACIOCRrs> {
        ACPDD_W::new(self, 3)
    }
    ///Bit 4 - ACPDR
    #[inline(always)]
    pub fn acpdr(&mut self) -> ACPDR_W<'_, ACIOCRrs> {
        ACPDR_W::new(self, 4)
    }
    ///Bits 5:7 - CKODT
    #[inline(always)]
    pub fn ckodt(&mut self) -> CKODT_W<'_, ACIOCRrs> {
        CKODT_W::new(self, 5)
    }
    ///Bits 8:10 - CKPDD
    #[inline(always)]
    pub fn ckpdd(&mut self) -> CKPDD_W<'_, ACIOCRrs> {
        CKPDD_W::new(self, 8)
    }
    ///Bits 11:13 - CKPDR
    #[inline(always)]
    pub fn ckpdr(&mut self) -> CKPDR_W<'_, ACIOCRrs> {
        CKPDR_W::new(self, 11)
    }
    ///Bit 14 - RANKODT
    #[inline(always)]
    pub fn rankodt(&mut self) -> RANKODT_W<'_, ACIOCRrs> {
        RANKODT_W::new(self, 14)
    }
    ///Bit 18 - CSPDD
    #[inline(always)]
    pub fn cspdd(&mut self) -> CSPDD_W<'_, ACIOCRrs> {
        CSPDD_W::new(self, 18)
    }
    ///Bit 22 - RANKPDR
    #[inline(always)]
    pub fn rankpdr(&mut self) -> RANKPDR_W<'_, ACIOCRrs> {
        RANKPDR_W::new(self, 22)
    }
    ///Bit 26 - RSTODT
    #[inline(always)]
    pub fn rstodt(&mut self) -> RSTODT_W<'_, ACIOCRrs> {
        RSTODT_W::new(self, 26)
    }
    ///Bit 27 - RSTPDD
    #[inline(always)]
    pub fn rstpdd(&mut self) -> RSTPDD_W<'_, ACIOCRrs> {
        RSTPDD_W::new(self, 27)
    }
    ///Bit 28 - RSTPDR
    #[inline(always)]
    pub fn rstpdr(&mut self) -> RSTPDR_W<'_, ACIOCRrs> {
        RSTPDR_W::new(self, 28)
    }
    ///Bit 29 - RSTIOM
    #[inline(always)]
    pub fn rstiom(&mut self) -> RSTIOM_W<'_, ACIOCRrs> {
        RSTIOM_W::new(self, 29)
    }
    ///Bits 30:31 - ACSR
    #[inline(always)]
    pub fn acsr(&mut self) -> ACSR_W<'_, ACIOCRrs> {
        ACSR_W::new(self, 30)
    }
}
/**DDRPHYC ACIOC register

You can [`read`](crate::Reg::read) this register and get [`aciocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aciocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ACIOCR)*/
pub struct ACIOCRrs;
impl crate::RegisterSpec for ACIOCRrs {
    type Ux = u32;
}
///`read()` method returns [`aciocr::R`](R) reader structure
impl crate::Readable for ACIOCRrs {}
///`write(|w| ..)` method takes [`aciocr::W`](W) writer structure
impl crate::Writable for ACIOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACIOCR to value 0x33c0_3812
impl crate::Resettable for ACIOCRrs {
    const RESET_VALUE: u32 = 0x33c0_3812;
}
