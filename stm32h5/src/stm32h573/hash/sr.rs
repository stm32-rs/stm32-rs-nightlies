#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `DINIS` reader - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
pub type DINIS_R = crate::BitReader;
#[doc = "Field `DINIS` writer - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
pub type DINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIS` reader - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
pub type DCIS_R = crate::BitReader;
#[doc = "Field `DCIS` writer - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
pub type DCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAS` reader - DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit."]
pub type DMAS_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `NBWP` reader - Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1."]
pub type NBWP_R = crate::FieldReader;
#[doc = "Field `DINNE` reader - DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1."]
pub type DINNE_R = crate::BitReader;
#[doc = "Field `NBWE` reader - Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends."]
pub type NBWE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit."]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1."]
    #[inline(always)]
    pub fn nbwp(&self) -> NBWP_R {
        NBWP_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1."]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends."]
    #[inline(always)]
    pub fn nbwe(&self) -> NBWE_R {
        NBWE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
    #[inline(always)]
    #[must_use]
    pub fn dinis(&mut self) -> DINIS_W<SRrs> {
        DINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn dcis(&mut self) -> DCIS_W<SRrs> {
        DCIS_W::new(self, 1)
    }
}
#[doc = "HASH status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x0011_0001"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x0011_0001;
}
