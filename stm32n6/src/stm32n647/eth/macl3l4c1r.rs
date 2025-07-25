///Register `MACL3L4C1R` reader
pub type R = crate::R<MACL3L4C1Rrs>;
///Register `MACL3L4C1R` writer
pub type W = crate::W<MACL3L4C1Rrs>;
///Field `L3PEN1` reader - Layer 3 Protocol Enable
pub type L3PEN1_R = crate::BitReader;
///Field `L3PEN1` writer - Layer 3 Protocol Enable
pub type L3PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3SAM1` reader - Layer 3 IP SA Match Enable
pub type L3SAM1_R = crate::BitReader;
///Field `L3SAM1` writer - Layer 3 IP SA Match Enable
pub type L3SAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3SAIM1` reader - Layer 3 IP SA Inverse Match Enable
pub type L3SAIM1_R = crate::BitReader;
///Field `L3SAIM1` writer - Layer 3 IP SA Inverse Match Enable
pub type L3SAIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3DAM1` reader - Layer 3 IP DA Match Enable
pub type L3DAM1_R = crate::BitReader;
///Field `L3DAM1` writer - Layer 3 IP DA Match Enable
pub type L3DAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3DAIM1` reader - Layer 3 IP DA Inverse Match Enable
pub type L3DAIM1_R = crate::BitReader;
///Field `L3DAIM1` writer - Layer 3 IP DA Inverse Match Enable
pub type L3DAIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3HSBM1` reader - Layer 3 IP SA Higher Bits Match
pub type L3HSBM1_R = crate::FieldReader;
///Field `L3HSBM1` writer - Layer 3 IP SA Higher Bits Match
pub type L3HSBM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L3HDBM1` reader - Layer 3 IP DA higher bits match
pub type L3HDBM1_R = crate::FieldReader;
///Field `L3HDBM1` writer - Layer 3 IP DA higher bits match
pub type L3HDBM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L4PEN1` reader - Layer 4 Protocol Enable
pub type L4PEN1_R = crate::BitReader;
///Field `L4PEN1` writer - Layer 4 Protocol Enable
pub type L4PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4SPM1` reader - Layer 4 Source Port Match Enable
pub type L4SPM1_R = crate::BitReader;
///Field `L4SPM1` writer - Layer 4 Source Port Match Enable
pub type L4SPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4SPIM1` reader - Layer 4 Source Port Inverse Match Enable
pub type L4SPIM1_R = crate::BitReader;
///Field `L4SPIM1` writer - Layer 4 Source Port Inverse Match Enable
pub type L4SPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4DPM1` reader - Layer 4 Destination Port Match Enable
pub type L4DPM1_R = crate::BitReader;
///Field `L4DPM1` writer - Layer 4 Destination Port Match Enable
pub type L4DPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4DPIM1` reader - Layer 4 Destination Port Inverse Match Enable
pub type L4DPIM1_R = crate::BitReader;
///Field `L4DPIM1` writer - Layer 4 Destination Port Inverse Match Enable
pub type L4DPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMCHN1` reader - DMA Channel Number
pub type DMCHN1_R = crate::BitReader;
///Field `DMCHN1` writer - DMA Channel Number
pub type DMCHN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMCHEN1` reader - DMA Channel Select Enable
pub type DMCHEN1_R = crate::BitReader;
///Field `DMCHEN1` writer - DMA Channel Select Enable
pub type DMCHEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Layer 3 Protocol Enable
    #[inline(always)]
    pub fn l3pen1(&self) -> L3PEN1_R {
        L3PEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Layer 3 IP SA Match Enable
    #[inline(always)]
    pub fn l3sam1(&self) -> L3SAM1_R {
        L3SAM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Layer 3 IP SA Inverse Match Enable
    #[inline(always)]
    pub fn l3saim1(&self) -> L3SAIM1_R {
        L3SAIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Layer 3 IP DA Match Enable
    #[inline(always)]
    pub fn l3dam1(&self) -> L3DAM1_R {
        L3DAM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Layer 3 IP DA Inverse Match Enable
    #[inline(always)]
    pub fn l3daim1(&self) -> L3DAIM1_R {
        L3DAIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - Layer 3 IP SA Higher Bits Match
    #[inline(always)]
    pub fn l3hsbm1(&self) -> L3HSBM1_R {
        L3HSBM1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - Layer 3 IP DA higher bits match
    #[inline(always)]
    pub fn l3hdbm1(&self) -> L3HDBM1_R {
        L3HDBM1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bit 16 - Layer 4 Protocol Enable
    #[inline(always)]
    pub fn l4pen1(&self) -> L4PEN1_R {
        L4PEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Layer 4 Source Port Match Enable
    #[inline(always)]
    pub fn l4spm1(&self) -> L4SPM1_R {
        L4SPM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Layer 4 Source Port Inverse Match Enable
    #[inline(always)]
    pub fn l4spim1(&self) -> L4SPIM1_R {
        L4SPIM1_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Layer 4 Destination Port Match Enable
    #[inline(always)]
    pub fn l4dpm1(&self) -> L4DPM1_R {
        L4DPM1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Layer 4 Destination Port Inverse Match Enable
    #[inline(always)]
    pub fn l4dpim1(&self) -> L4DPIM1_R {
        L4DPIM1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - DMA Channel Number
    #[inline(always)]
    pub fn dmchn1(&self) -> DMCHN1_R {
        DMCHN1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - DMA Channel Select Enable
    #[inline(always)]
    pub fn dmchen1(&self) -> DMCHEN1_R {
        DMCHEN1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3L4C1R")
            .field("l3pen1", &self.l3pen1())
            .field("l3sam1", &self.l3sam1())
            .field("l3saim1", &self.l3saim1())
            .field("l3dam1", &self.l3dam1())
            .field("l3daim1", &self.l3daim1())
            .field("l3hsbm1", &self.l3hsbm1())
            .field("l3hdbm1", &self.l3hdbm1())
            .field("l4pen1", &self.l4pen1())
            .field("l4spm1", &self.l4spm1())
            .field("l4spim1", &self.l4spim1())
            .field("l4dpm1", &self.l4dpm1())
            .field("l4dpim1", &self.l4dpim1())
            .field("dmchn1", &self.dmchn1())
            .field("dmchen1", &self.dmchen1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Layer 3 Protocol Enable
    #[inline(always)]
    pub fn l3pen1(&mut self) -> L3PEN1_W<MACL3L4C1Rrs> {
        L3PEN1_W::new(self, 0)
    }
    ///Bit 2 - Layer 3 IP SA Match Enable
    #[inline(always)]
    pub fn l3sam1(&mut self) -> L3SAM1_W<MACL3L4C1Rrs> {
        L3SAM1_W::new(self, 2)
    }
    ///Bit 3 - Layer 3 IP SA Inverse Match Enable
    #[inline(always)]
    pub fn l3saim1(&mut self) -> L3SAIM1_W<MACL3L4C1Rrs> {
        L3SAIM1_W::new(self, 3)
    }
    ///Bit 4 - Layer 3 IP DA Match Enable
    #[inline(always)]
    pub fn l3dam1(&mut self) -> L3DAM1_W<MACL3L4C1Rrs> {
        L3DAM1_W::new(self, 4)
    }
    ///Bit 5 - Layer 3 IP DA Inverse Match Enable
    #[inline(always)]
    pub fn l3daim1(&mut self) -> L3DAIM1_W<MACL3L4C1Rrs> {
        L3DAIM1_W::new(self, 5)
    }
    ///Bits 6:10 - Layer 3 IP SA Higher Bits Match
    #[inline(always)]
    pub fn l3hsbm1(&mut self) -> L3HSBM1_W<MACL3L4C1Rrs> {
        L3HSBM1_W::new(self, 6)
    }
    ///Bits 11:15 - Layer 3 IP DA higher bits match
    #[inline(always)]
    pub fn l3hdbm1(&mut self) -> L3HDBM1_W<MACL3L4C1Rrs> {
        L3HDBM1_W::new(self, 11)
    }
    ///Bit 16 - Layer 4 Protocol Enable
    #[inline(always)]
    pub fn l4pen1(&mut self) -> L4PEN1_W<MACL3L4C1Rrs> {
        L4PEN1_W::new(self, 16)
    }
    ///Bit 18 - Layer 4 Source Port Match Enable
    #[inline(always)]
    pub fn l4spm1(&mut self) -> L4SPM1_W<MACL3L4C1Rrs> {
        L4SPM1_W::new(self, 18)
    }
    ///Bit 19 - Layer 4 Source Port Inverse Match Enable
    #[inline(always)]
    pub fn l4spim1(&mut self) -> L4SPIM1_W<MACL3L4C1Rrs> {
        L4SPIM1_W::new(self, 19)
    }
    ///Bit 20 - Layer 4 Destination Port Match Enable
    #[inline(always)]
    pub fn l4dpm1(&mut self) -> L4DPM1_W<MACL3L4C1Rrs> {
        L4DPM1_W::new(self, 20)
    }
    ///Bit 21 - Layer 4 Destination Port Inverse Match Enable
    #[inline(always)]
    pub fn l4dpim1(&mut self) -> L4DPIM1_W<MACL3L4C1Rrs> {
        L4DPIM1_W::new(self, 21)
    }
    ///Bit 24 - DMA Channel Number
    #[inline(always)]
    pub fn dmchn1(&mut self) -> DMCHN1_W<MACL3L4C1Rrs> {
        DMCHN1_W::new(self, 24)
    }
    ///Bit 28 - DMA Channel Select Enable
    #[inline(always)]
    pub fn dmchen1(&mut self) -> DMCHEN1_W<MACL3L4C1Rrs> {
        DMCHEN1_W::new(self, 28)
    }
}
/**L3 and L4 control 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3l4c1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MACL3L4C1R)*/
pub struct MACL3L4C1Rrs;
impl crate::RegisterSpec for MACL3L4C1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3l4c1r::R`](R) reader structure
impl crate::Readable for MACL3L4C1Rrs {}
///`write(|w| ..)` method takes [`macl3l4c1r::W`](W) writer structure
impl crate::Writable for MACL3L4C1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3L4C1R to value 0
impl crate::Resettable for MACL3L4C1Rrs {}
