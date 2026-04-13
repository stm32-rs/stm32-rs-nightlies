///Register `REGCR4` reader
pub type R = crate::R<REGCR4rs>;
///Register `REGCR4` writer
pub type W = crate::W<REGCR4rs>;
///Field `BREN` reader - Base region enable
pub type BREN_R = crate::BitReader;
///Field `BREN` writer - Base region enable
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTXID` reader - Context ID
pub type CTXID_R = crate::FieldReader;
///Field `CTXID` writer - Context ID
pub type CTXID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ENC` reader - Encrypted region
pub type ENC_R = crate::FieldReader;
///Field `ENC` writer - Encrypted region
pub type ENC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Base region enable
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bits 9:10 - Context ID
    #[inline(always)]
    pub fn ctxid(&self) -> CTXID_R {
        CTXID_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 14:15 - Encrypted region
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCR4")
            .field("bren", &self.bren())
            .field("ctxid", &self.ctxid())
            .field("enc", &self.enc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Base region enable
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W<'_, REGCR4rs> {
        BREN_W::new(self, 0)
    }
    ///Bits 9:10 - Context ID
    #[inline(always)]
    pub fn ctxid(&mut self) -> CTXID_W<'_, REGCR4rs> {
        CTXID_W::new(self, 9)
    }
    ///Bits 14:15 - Encrypted region
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W<'_, REGCR4rs> {
        ENC_W::new(self, 14)
    }
}
/**MCE region 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:REGCR4)*/
pub struct REGCR4rs;
impl crate::RegisterSpec for REGCR4rs {
    type Ux = u32;
}
///`read()` method returns [`regcr4::R`](R) reader structure
impl crate::Readable for REGCR4rs {}
///`write(|w| ..)` method takes [`regcr4::W`](W) writer structure
impl crate::Writable for REGCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGCR4 to value 0
impl crate::Resettable for REGCR4rs {}
