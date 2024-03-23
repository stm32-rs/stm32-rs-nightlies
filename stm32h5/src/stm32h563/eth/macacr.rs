#[doc = "Register `MACACR` reader"]
pub type R = crate::R<MACACRrs>;
#[doc = "Register `MACACR` writer"]
pub type W = crate::W<MACACRrs>;
#[doc = "Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
pub type ATSFC_R = crate::BitReader;
#[doc = "Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
pub type ATSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN0` reader - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN0_R = crate::BitReader;
#[doc = "Field `ATSEN0` writer - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN1` reader - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN1_R = crate::BitReader;
#[doc = "Field `ATSEN1` writer - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN2` reader - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN2_R = crate::BitReader;
#[doc = "Field `ATSEN2` writer - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN3` reader - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN3_R = crate::BitReader;
#[doc = "Field `ATSEN3` writer - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
pub type ATSEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, the auxiliary snapshots are stored in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn atsfc(&mut self) -> ATSFC_W<MACACRrs> {
        ATSFC_W::new(self, 0)
    }
    #[doc = "Bit 4 - Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg0 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen0(&mut self) -> ATSEN0_W<MACACRrs> {
        ATSEN0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg1 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen1(&mut self) -> ATSEN1_W<MACACRrs> {
        ATSEN1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg2 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen2(&mut self) -> ATSEN2_W<MACACRrs> {
        ATSEN2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3. When this bit is set, the auxiliary snapshot of the event on eth_ptp_trg3 input is enabled. When this bit is reset, the events on this input are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn atsen3(&mut self) -> ATSEN3_W<MACACRrs> {
        ATSEN3_W::new(self, 7)
    }
}
#[doc = "Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACACRrs;
impl crate::RegisterSpec for MACACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macacr::R`](R) reader structure"]
impl crate::Readable for MACACRrs {}
#[doc = "`write(|w| ..)` method takes [`macacr::W`](W) writer structure"]
impl crate::Writable for MACACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACACR to value 0"]
impl crate::Resettable for MACACRrs {
    const RESET_VALUE: u32 = 0;
}
