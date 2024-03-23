#[doc = "Register `MACTXTSSNR` reader"]
pub type R = crate::R<MACTXTSSNRrs>;
#[doc = "Register `MACTXTSSNR` writer"]
pub type W = crate::W<MACTXTSSNRrs>;
#[doc = "Field `TXTSSLO` reader - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXTSSLO_R = crate::FieldReader<u32>;
#[doc = "Field `TXTSSLO` writer - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp."]
pub type TXTSSLO_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TXTSSMIS` reader - Transmit Timestamp Status Missed When this bit is set, it indicates one of the following: The timestamp of the current packet is ignored if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is reset The timestamp of the previous packet is overwritten with timestamp of the current packet if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is set."]
pub type TXTSSMIS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp."]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Transmit Timestamp Status Missed When this bit is set, it indicates one of the following: The timestamp of the current packet is ignored if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is reset The timestamp of the previous packet is overwritten with timestamp of the current packet if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is set."]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp."]
    #[inline(always)]
    #[must_use]
    pub fn txtsslo(&mut self) -> TXTSSLO_W<MACTXTSSNRrs> {
        TXTSSLO_W::new(self, 0)
    }
}
#[doc = "Tx timestamp status nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactxtssnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactxtssnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTXTSSNRrs;
impl crate::RegisterSpec for MACTXTSSNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactxtssnr::R`](R) reader structure"]
impl crate::Readable for MACTXTSSNRrs {}
#[doc = "`write(|w| ..)` method takes [`mactxtssnr::W`](W) writer structure"]
impl crate::Writable for MACTXTSSNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTXTSSNR to value 0"]
impl crate::Resettable for MACTXTSSNRrs {
    const RESET_VALUE: u32 = 0;
}
