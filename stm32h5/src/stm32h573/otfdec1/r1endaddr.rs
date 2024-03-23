#[doc = "Register `R1ENDADDR` reader"]
pub type R = crate::R<R1ENDADDRrs>;
#[doc = "Register `R1ENDADDR` writer"]
pub type W = crate::W<R1ENDADDRrs>;
#[doc = "Field `REGx_END_ADDR` reader - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones."]
pub type REGX_END_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `REGx_END_ADDR` writer - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones."]
pub type REGX_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones."]
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AHB end address This register must be written before the region corresponding REG_EN bit in the OTFDEC_RxCFGR register is set, and OTFDEC_RxENDADDR must be strictly greater than OTFDEC_RxSTARTADDR to be valid. Writing to this register is discarded if performed while the region CONFIGLOCK bit in OTFDEC_RxCFGR is set. Note: When determining the region the first 12 bits (lsb) and the last 4 bits (msb) are ignored. When this register is accessed in read the 4 msb bits return zeros and the 12 lsb bits return ones."]
    #[inline(always)]
    #[must_use]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W<R1ENDADDRrs> {
        REGX_END_ADDR_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 1 end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1endaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1endaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R1ENDADDRrs;
impl crate::RegisterSpec for R1ENDADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r1endaddr::R`](R) reader structure"]
impl crate::Readable for R1ENDADDRrs {}
#[doc = "`write(|w| ..)` method takes [`r1endaddr::W`](W) writer structure"]
impl crate::Writable for R1ENDADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R1ENDADDR to value 0x0fff"]
impl crate::Resettable for R1ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
