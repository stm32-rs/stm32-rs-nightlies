///Register `MACL3L4C0R` reader
pub type R = crate::R<MACL3L4C0Rrs>;
///Register `MACL3L4C0R` writer
pub type W = crate::W<MACL3L4C0Rrs>;
///Field `L3PEN0` reader - Layer 3 Protocol Enable
pub type L3PEN0_R = crate::BitReader;
///Field `L3PEN0` writer - Layer 3 Protocol Enable
pub type L3PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3SAM0` reader - Layer 3 IP SA Match Enable
pub type L3SAM0_R = crate::BitReader;
///Field `L3SAM0` writer - Layer 3 IP SA Match Enable
pub type L3SAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3SAIM0` reader - Layer 3 IP SA Inverse Match Enable
pub type L3SAIM0_R = crate::BitReader;
///Field `L3SAIM0` writer - Layer 3 IP SA Inverse Match Enable
pub type L3SAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3DAM0` reader - Layer 3 IP DA Match Enable
pub type L3DAM0_R = crate::BitReader;
///Field `L3DAM0` writer - Layer 3 IP DA Match Enable
pub type L3DAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3DAIM0` reader - Layer 3 IP DA Inverse Match Enable
pub type L3DAIM0_R = crate::BitReader;
///Field `L3DAIM0` writer - Layer 3 IP DA Inverse Match Enable
pub type L3DAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3HSBM0` reader - Layer 3 IP SA higher bits match
pub type L3HSBM0_R = crate::FieldReader;
///Field `L3HSBM0` writer - Layer 3 IP SA higher bits match
pub type L3HSBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L3HDBM0` reader - Layer 3 IP DA higher bits match
pub type L3HDBM0_R = crate::FieldReader;
///Field `L3HDBM0` writer - Layer 3 IP DA higher bits match
pub type L3HDBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L4PEN0` reader - Layer 4 Protocol Enable
pub type L4PEN0_R = crate::BitReader;
///Field `L4PEN0` writer - Layer 4 Protocol Enable
pub type L4PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4SPM0` reader - Layer 4 Source Port Match Enable
pub type L4SPM0_R = crate::BitReader;
///Field `L4SPM0` writer - Layer 4 Source Port Match Enable
pub type L4SPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4SPIM0` reader - Layer 4 Source Port Inverse Match Enable
pub type L4SPIM0_R = crate::BitReader;
///Field `L4SPIM0` writer - Layer 4 Source Port Inverse Match Enable
pub type L4SPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4DPM0` reader - Layer 4 Destination Port Match Enable
pub type L4DPM0_R = crate::BitReader;
///Field `L4DPM0` writer - Layer 4 Destination Port Match Enable
pub type L4DPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4DPIM0` reader - Layer 4 Destination Port Inverse Match Enable
pub type L4DPIM0_R = crate::BitReader;
///Field `L4DPIM0` writer - Layer 4 Destination Port Inverse Match Enable
pub type L4DPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMCHN0` reader - DMA Channel Number
pub type DMCHN0_R = crate::BitReader;
///Field `DMCHN0` writer - DMA Channel Number
pub type DMCHN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMCHEN0` reader - DMA Channel Select Enable
pub type DMCHEN0_R = crate::BitReader;
///Field `DMCHEN0` writer - DMA Channel Select Enable
pub type DMCHEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Layer 3 Protocol Enable
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Layer 3 IP SA Match Enable
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Layer 3 IP SA Inverse Match Enable
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Layer 3 IP DA Match Enable
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Layer 3 IP DA Inverse Match Enable
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - Layer 3 IP SA higher bits match
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - Layer 3 IP DA higher bits match
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bit 16 - Layer 4 Protocol Enable
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Layer 4 Source Port Match Enable
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Layer 4 Source Port Inverse Match Enable
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Layer 4 Destination Port Match Enable
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Layer 4 Destination Port Inverse Match Enable
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - DMA Channel Number
    #[inline(always)]
    pub fn dmchn0(&self) -> DMCHN0_R {
        DMCHN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - DMA Channel Select Enable
    #[inline(always)]
    pub fn dmchen0(&self) -> DMCHEN0_R {
        DMCHEN0_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3L4C0R")
            .field("l3pen0", &self.l3pen0())
            .field("l3sam0", &self.l3sam0())
            .field("l3saim0", &self.l3saim0())
            .field("l3dam0", &self.l3dam0())
            .field("l3daim0", &self.l3daim0())
            .field("l3hsbm0", &self.l3hsbm0())
            .field("l3hdbm0", &self.l3hdbm0())
            .field("l4pen0", &self.l4pen0())
            .field("l4spm0", &self.l4spm0())
            .field("l4spim0", &self.l4spim0())
            .field("l4dpm0", &self.l4dpm0())
            .field("l4dpim0", &self.l4dpim0())
            .field("dmchn0", &self.dmchn0())
            .field("dmchen0", &self.dmchen0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Layer 3 Protocol Enable
    #[inline(always)]
    pub fn l3pen0(&mut self) -> L3PEN0_W<'_, MACL3L4C0Rrs> {
        L3PEN0_W::new(self, 0)
    }
    ///Bit 2 - Layer 3 IP SA Match Enable
    #[inline(always)]
    pub fn l3sam0(&mut self) -> L3SAM0_W<'_, MACL3L4C0Rrs> {
        L3SAM0_W::new(self, 2)
    }
    ///Bit 3 - Layer 3 IP SA Inverse Match Enable
    #[inline(always)]
    pub fn l3saim0(&mut self) -> L3SAIM0_W<'_, MACL3L4C0Rrs> {
        L3SAIM0_W::new(self, 3)
    }
    ///Bit 4 - Layer 3 IP DA Match Enable
    #[inline(always)]
    pub fn l3dam0(&mut self) -> L3DAM0_W<'_, MACL3L4C0Rrs> {
        L3DAM0_W::new(self, 4)
    }
    ///Bit 5 - Layer 3 IP DA Inverse Match Enable
    #[inline(always)]
    pub fn l3daim0(&mut self) -> L3DAIM0_W<'_, MACL3L4C0Rrs> {
        L3DAIM0_W::new(self, 5)
    }
    ///Bits 6:10 - Layer 3 IP SA higher bits match
    #[inline(always)]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W<'_, MACL3L4C0Rrs> {
        L3HSBM0_W::new(self, 6)
    }
    ///Bits 11:15 - Layer 3 IP DA higher bits match
    #[inline(always)]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W<'_, MACL3L4C0Rrs> {
        L3HDBM0_W::new(self, 11)
    }
    ///Bit 16 - Layer 4 Protocol Enable
    #[inline(always)]
    pub fn l4pen0(&mut self) -> L4PEN0_W<'_, MACL3L4C0Rrs> {
        L4PEN0_W::new(self, 16)
    }
    ///Bit 18 - Layer 4 Source Port Match Enable
    #[inline(always)]
    pub fn l4spm0(&mut self) -> L4SPM0_W<'_, MACL3L4C0Rrs> {
        L4SPM0_W::new(self, 18)
    }
    ///Bit 19 - Layer 4 Source Port Inverse Match Enable
    #[inline(always)]
    pub fn l4spim0(&mut self) -> L4SPIM0_W<'_, MACL3L4C0Rrs> {
        L4SPIM0_W::new(self, 19)
    }
    ///Bit 20 - Layer 4 Destination Port Match Enable
    #[inline(always)]
    pub fn l4dpm0(&mut self) -> L4DPM0_W<'_, MACL3L4C0Rrs> {
        L4DPM0_W::new(self, 20)
    }
    ///Bit 21 - Layer 4 Destination Port Inverse Match Enable
    #[inline(always)]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W<'_, MACL3L4C0Rrs> {
        L4DPIM0_W::new(self, 21)
    }
    ///Bit 24 - DMA Channel Number
    #[inline(always)]
    pub fn dmchn0(&mut self) -> DMCHN0_W<'_, MACL3L4C0Rrs> {
        DMCHN0_W::new(self, 24)
    }
    ///Bit 28 - DMA Channel Select Enable
    #[inline(always)]
    pub fn dmchen0(&mut self) -> DMCHEN0_W<'_, MACL3L4C0Rrs> {
        DMCHEN0_W::new(self, 28)
    }
}
/**L3 and L4 control 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACL3L4C0R)*/
pub struct MACL3L4C0Rrs;
impl crate::RegisterSpec for MACL3L4C0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3l4c0r::R`](R) reader structure
impl crate::Readable for MACL3L4C0Rrs {}
///`write(|w| ..)` method takes [`macl3l4c0r::W`](W) writer structure
impl crate::Writable for MACL3L4C0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3L4C0R to value 0
impl crate::Resettable for MACL3L4C0Rrs {}
