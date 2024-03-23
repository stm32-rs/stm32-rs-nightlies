#[doc = "Register `HSPI_DCR3` reader"]
pub type R = crate::R<HSPI_DCR3rs>;
#[doc = "Register `HSPI_DCR3` writer"]
pub type W = crate::W<HSPI_DCR3rs>;
#[doc = "Field `MAXTRAN` reader - Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes"]
pub type MAXTRAN_R = crate::FieldReader;
#[doc = "Field `MAXTRAN` writer - Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes"]
pub type MAXTRAN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSBOUND` reader - CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes"]
pub type CSBOUND_R = crate::FieldReader;
#[doc = "Field `CSBOUND` writer - CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes"]
pub type CSBOUND_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes"]
    #[inline(always)]
    pub fn maxtran(&self) -> MAXTRAN_R {
        MAXTRAN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes"]
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn maxtran(&mut self) -> MAXTRAN_W<HSPI_DCR3rs> {
        MAXTRAN_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes"]
    #[inline(always)]
    #[must_use]
    pub fn csbound(&mut self) -> CSBOUND_W<HSPI_DCR3rs> {
        CSBOUND_W::new(self, 16)
    }
}
#[doc = "HSPI device configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_dcr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_dcr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_DCR3rs;
impl crate::RegisterSpec for HSPI_DCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_dcr3::R`](R) reader structure"]
impl crate::Readable for HSPI_DCR3rs {}
#[doc = "`write(|w| ..)` method takes [`hspi_dcr3::W`](W) writer structure"]
impl crate::Writable for HSPI_DCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_DCR3 to value 0"]
impl crate::Resettable for HSPI_DCR3rs {
    const RESET_VALUE: u32 = 0;
}
