///Register `MACRXQC1R` reader
pub type R = crate::R<MACRXQC1Rrs>;
///Register `MACRXQC1R` writer
pub type W = crate::W<MACRXQC1Rrs>;
///Field `AVCPQ0` reader - AV Untagged Control Packets Queue
pub type AVCPQ0_R = crate::BitReader;
///Field `AVCPQ0` writer - AV Untagged Control Packets Queue
pub type AVCPQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVCPQ1` reader - AV Untagged Control Packets Queue
pub type AVCPQ1_R = crate::BitReader;
///Field `AVCPQ1` writer - AV Untagged Control Packets Queue
pub type AVCPQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVCPQ2` reader - AV Untagged Control Packets Queue
pub type AVCPQ2_R = crate::BitReader;
///Field `AVCPQ2` writer - AV Untagged Control Packets Queue
pub type AVCPQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTPQ` reader - PTP Packets Queue
pub type PTPQ_R = crate::FieldReader;
///Field `PTPQ` writer - PTP Packets Queue
pub type PTPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UPQ` reader - Untagged Packet Queue
pub type UPQ_R = crate::FieldReader;
///Field `UPQ` writer - Untagged Packet Queue
pub type UPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCBCQ` reader - Multicast and Broadcast Queue
pub type MCBCQ_R = crate::FieldReader;
///Field `MCBCQ` writer - Multicast and Broadcast Queue
pub type MCBCQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCBCQEN` reader - Multicast and Broadcast Queue Enable
pub type MCBCQEN_R = crate::BitReader;
///Field `MCBCQEN` writer - Multicast and Broadcast Queue Enable
pub type MCBCQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TACPQE` reader - Tagged AV Control Packets Queuing Enable
pub type TACPQE_R = crate::BitReader;
///Field `TACPQE` writer - Tagged AV Control Packets Queuing Enable
pub type TACPQE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPQC` reader - Tagged PTP over Ethernet Packets Queuing Control
pub type TPQC_R = crate::FieldReader;
///Field `TPQC` writer - Tagged PTP over Ethernet Packets Queuing Control
pub type TPQC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FPRQ0` reader - Frame Preemption Residue Queue
pub type FPRQ0_R = crate::BitReader;
///Field `FPRQ0` writer - Frame Preemption Residue Queue
pub type FPRQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPRQ1` reader - Frame Preemption Residue Queue
pub type FPRQ1_R = crate::BitReader;
///Field `FPRQ1` writer - Frame Preemption Residue Queue
pub type FPRQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPRQ2` reader - Frame Preemption Residue Queue
pub type FPRQ2_R = crate::BitReader;
///Field `FPRQ2` writer - Frame Preemption Residue Queue
pub type FPRQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OMCBCQ` reader - Overriding MC-BC queue priority select
pub type OMCBCQ_R = crate::BitReader;
///Field `OMCBCQ` writer - Overriding MC-BC queue priority select
pub type OMCBCQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBRQE` reader - Type Field Based Rx Queuing Enable
pub type TBRQE_R = crate::BitReader;
///Field `TBRQE` writer - Type Field Based Rx Queuing Enable
pub type TBRQE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AV Untagged Control Packets Queue
    #[inline(always)]
    pub fn avcpq0(&self) -> AVCPQ0_R {
        AVCPQ0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AV Untagged Control Packets Queue
    #[inline(always)]
    pub fn avcpq1(&self) -> AVCPQ1_R {
        AVCPQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AV Untagged Control Packets Queue
    #[inline(always)]
    pub fn avcpq2(&self) -> AVCPQ2_R {
        AVCPQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - PTP Packets Queue
    #[inline(always)]
    pub fn ptpq(&self) -> PTPQ_R {
        PTPQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - Untagged Packet Queue
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - Multicast and Broadcast Queue
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - Multicast and Broadcast Queue Enable
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Tagged AV Control Packets Queuing Enable
    #[inline(always)]
    pub fn tacpqe(&self) -> TACPQE_R {
        TACPQE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Tagged PTP over Ethernet Packets Queuing Control
    #[inline(always)]
    pub fn tpqc(&self) -> TPQC_R {
        TPQC_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Frame Preemption Residue Queue
    #[inline(always)]
    pub fn fprq0(&self) -> FPRQ0_R {
        FPRQ0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Frame Preemption Residue Queue
    #[inline(always)]
    pub fn fprq1(&self) -> FPRQ1_R {
        FPRQ1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Frame Preemption Residue Queue
    #[inline(always)]
    pub fn fprq2(&self) -> FPRQ2_R {
        FPRQ2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Overriding MC-BC queue priority select
    #[inline(always)]
    pub fn omcbcq(&self) -> OMCBCQ_R {
        OMCBCQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Type Field Based Rx Queuing Enable
    #[inline(always)]
    pub fn tbrqe(&self) -> TBRQE_R {
        TBRQE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRXQC1R")
            .field("avcpq0", &self.avcpq0())
            .field("avcpq1", &self.avcpq1())
            .field("avcpq2", &self.avcpq2())
            .field("ptpq", &self.ptpq())
            .field("upq", &self.upq())
            .field("mcbcq", &self.mcbcq())
            .field("mcbcqen", &self.mcbcqen())
            .field("tacpqe", &self.tacpqe())
            .field("tpqc", &self.tpqc())
            .field("fprq0", &self.fprq0())
            .field("fprq1", &self.fprq1())
            .field("fprq2", &self.fprq2())
            .field("omcbcq", &self.omcbcq())
            .field("tbrqe", &self.tbrqe())
            .finish()
    }
}
impl W {
    ///Bit 0 - AV Untagged Control Packets Queue
    #[inline(always)]
    pub fn avcpq0(&mut self) -> AVCPQ0_W<'_, MACRXQC1Rrs> {
        AVCPQ0_W::new(self, 0)
    }
    ///Bit 1 - AV Untagged Control Packets Queue
    #[inline(always)]
    pub fn avcpq1(&mut self) -> AVCPQ1_W<'_, MACRXQC1Rrs> {
        AVCPQ1_W::new(self, 1)
    }
    ///Bit 2 - AV Untagged Control Packets Queue
    #[inline(always)]
    pub fn avcpq2(&mut self) -> AVCPQ2_W<'_, MACRXQC1Rrs> {
        AVCPQ2_W::new(self, 2)
    }
    ///Bits 4:6 - PTP Packets Queue
    #[inline(always)]
    pub fn ptpq(&mut self) -> PTPQ_W<'_, MACRXQC1Rrs> {
        PTPQ_W::new(self, 4)
    }
    ///Bits 12:14 - Untagged Packet Queue
    #[inline(always)]
    pub fn upq(&mut self) -> UPQ_W<'_, MACRXQC1Rrs> {
        UPQ_W::new(self, 12)
    }
    ///Bits 16:18 - Multicast and Broadcast Queue
    #[inline(always)]
    pub fn mcbcq(&mut self) -> MCBCQ_W<'_, MACRXQC1Rrs> {
        MCBCQ_W::new(self, 16)
    }
    ///Bit 20 - Multicast and Broadcast Queue Enable
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W<'_, MACRXQC1Rrs> {
        MCBCQEN_W::new(self, 20)
    }
    ///Bit 21 - Tagged AV Control Packets Queuing Enable
    #[inline(always)]
    pub fn tacpqe(&mut self) -> TACPQE_W<'_, MACRXQC1Rrs> {
        TACPQE_W::new(self, 21)
    }
    ///Bits 22:23 - Tagged PTP over Ethernet Packets Queuing Control
    #[inline(always)]
    pub fn tpqc(&mut self) -> TPQC_W<'_, MACRXQC1Rrs> {
        TPQC_W::new(self, 22)
    }
    ///Bit 24 - Frame Preemption Residue Queue
    #[inline(always)]
    pub fn fprq0(&mut self) -> FPRQ0_W<'_, MACRXQC1Rrs> {
        FPRQ0_W::new(self, 24)
    }
    ///Bit 25 - Frame Preemption Residue Queue
    #[inline(always)]
    pub fn fprq1(&mut self) -> FPRQ1_W<'_, MACRXQC1Rrs> {
        FPRQ1_W::new(self, 25)
    }
    ///Bit 26 - Frame Preemption Residue Queue
    #[inline(always)]
    pub fn fprq2(&mut self) -> FPRQ2_W<'_, MACRXQC1Rrs> {
        FPRQ2_W::new(self, 26)
    }
    ///Bit 28 - Overriding MC-BC queue priority select
    #[inline(always)]
    pub fn omcbcq(&mut self) -> OMCBCQ_W<'_, MACRXQC1Rrs> {
        OMCBCQ_W::new(self, 28)
    }
    ///Bit 29 - Type Field Based Rx Queuing Enable
    #[inline(always)]
    pub fn tbrqe(&mut self) -> TBRQE_W<'_, MACRXQC1Rrs> {
        TBRQE_W::new(self, 29)
    }
}
/**Rx queue control 1 register

You can [`read`](crate::Reg::read) this register and get [`macrxqc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxqc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACRXQC1R)*/
pub struct MACRXQC1Rrs;
impl crate::RegisterSpec for MACRXQC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macrxqc1r::R`](R) reader structure
impl crate::Readable for MACRXQC1Rrs {}
///`write(|w| ..)` method takes [`macrxqc1r::W`](W) writer structure
impl crate::Writable for MACRXQC1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRXQC1R to value 0
impl crate::Resettable for MACRXQC1Rrs {}
