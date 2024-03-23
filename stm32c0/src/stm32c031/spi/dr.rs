#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DRrs>;
#[doc = "Field `DR` reader - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See ). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
pub type DR_R = crate::FieldReader<u16>;
#[doc = "Field `DR` writer - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See ). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See ). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data register Data received or to be transmitted The data register serves as an interface between the Rx and Tx FIFOs. When the data register is read, RxFIFO is accessed while the write to data register accesses TxFIFO (See ). Note: Data is always right-aligned. Unused bits are ignored when writing to the register, and read as zero when the register is read. The Rx threshold setting must always correspond with the read access currently used."]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<DRrs> {
        DR_W::new(self, 0)
    }
}
#[doc = "SPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u16 = 0;
}
