///Register `MACL3L4C0R` reader
pub type R = crate::R<MACL3L4C0Rrs>;
///Register `MACL3L4C0R` writer
pub type W = crate::W<MACL3L4C0Rrs>;
///Field `L3PEN0` reader - L3PEN0
pub type L3PEN0_R = crate::BitReader;
///Field `L3PEN0` writer - L3PEN0
pub type L3PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3SAM0` reader - L3SAM0
pub type L3SAM0_R = crate::BitReader;
///Field `L3SAM0` writer - L3SAM0
pub type L3SAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3SAIM0` reader - L3SAIM0
pub type L3SAIM0_R = crate::BitReader;
///Field `L3SAIM0` writer - L3SAIM0
pub type L3SAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3DAM0` reader - L3DAM0
pub type L3DAM0_R = crate::BitReader;
///Field `L3DAM0` writer - L3DAM0
pub type L3DAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3DAIM0` reader - L3DAIM0
pub type L3DAIM0_R = crate::BitReader;
///Field `L3DAIM0` writer - L3DAIM0
pub type L3DAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L3HSBM0` reader - L3HSBM0
pub type L3HSBM0_R = crate::FieldReader;
///Field `L3HSBM0` writer - L3HSBM0
pub type L3HSBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L3HDBM0` reader - L3HDBM0
pub type L3HDBM0_R = crate::FieldReader;
///Field `L3HDBM0` writer - L3HDBM0
pub type L3HDBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `L4PEN0` reader - L4PEN0
pub type L4PEN0_R = crate::BitReader;
///Field `L4PEN0` writer - L4PEN0
pub type L4PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4SPM0` reader - L4SPM0
pub type L4SPM0_R = crate::BitReader;
///Field `L4SPM0` writer - L4SPM0
pub type L4SPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4SPIM0` reader - L4SPIM0
pub type L4SPIM0_R = crate::BitReader;
///Field `L4SPIM0` writer - L4SPIM0
pub type L4SPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4DPM0` reader - L4DPM0
pub type L4DPM0_R = crate::BitReader;
///Field `L4DPM0` writer - L4DPM0
pub type L4DPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L4DPIM0` reader - L4DPIM0
pub type L4DPIM0_R = crate::BitReader;
///Field `L4DPIM0` writer - L4DPIM0
pub type L4DPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - L3PEN0
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - L3SAM0
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - L3SAIM0
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - L3DAM0
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - L3DAIM0
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - L3HSBM0
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - L3HDBM0
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bit 16 - L4PEN0
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - L4SPM0
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - L4SPIM0
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - L4DPM0
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - L4DPIM0
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 1) != 0)
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
            .finish()
    }
}
impl W {
    ///Bit 0 - L3PEN0
    #[inline(always)]
    pub fn l3pen0(&mut self) -> L3PEN0_W<'_, MACL3L4C0Rrs> {
        L3PEN0_W::new(self, 0)
    }
    ///Bit 2 - L3SAM0
    #[inline(always)]
    pub fn l3sam0(&mut self) -> L3SAM0_W<'_, MACL3L4C0Rrs> {
        L3SAM0_W::new(self, 2)
    }
    ///Bit 3 - L3SAIM0
    #[inline(always)]
    pub fn l3saim0(&mut self) -> L3SAIM0_W<'_, MACL3L4C0Rrs> {
        L3SAIM0_W::new(self, 3)
    }
    ///Bit 4 - L3DAM0
    #[inline(always)]
    pub fn l3dam0(&mut self) -> L3DAM0_W<'_, MACL3L4C0Rrs> {
        L3DAM0_W::new(self, 4)
    }
    ///Bit 5 - L3DAIM0
    #[inline(always)]
    pub fn l3daim0(&mut self) -> L3DAIM0_W<'_, MACL3L4C0Rrs> {
        L3DAIM0_W::new(self, 5)
    }
    ///Bits 6:10 - L3HSBM0
    #[inline(always)]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W<'_, MACL3L4C0Rrs> {
        L3HSBM0_W::new(self, 6)
    }
    ///Bits 11:15 - L3HDBM0
    #[inline(always)]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W<'_, MACL3L4C0Rrs> {
        L3HDBM0_W::new(self, 11)
    }
    ///Bit 16 - L4PEN0
    #[inline(always)]
    pub fn l4pen0(&mut self) -> L4PEN0_W<'_, MACL3L4C0Rrs> {
        L4PEN0_W::new(self, 16)
    }
    ///Bit 18 - L4SPM0
    #[inline(always)]
    pub fn l4spm0(&mut self) -> L4SPM0_W<'_, MACL3L4C0Rrs> {
        L4SPM0_W::new(self, 18)
    }
    ///Bit 19 - L4SPIM0
    #[inline(always)]
    pub fn l4spim0(&mut self) -> L4SPIM0_W<'_, MACL3L4C0Rrs> {
        L4SPIM0_W::new(self, 19)
    }
    ///Bit 20 - L4DPM0
    #[inline(always)]
    pub fn l4dpm0(&mut self) -> L4DPM0_W<'_, MACL3L4C0Rrs> {
        L4DPM0_W::new(self, 20)
    }
    ///Bit 21 - L4DPIM0
    #[inline(always)]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W<'_, MACL3L4C0Rrs> {
        L4DPIM0_W::new(self, 21)
    }
}
/**The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4. This register is reserved if the Layer 3 and Layer 4 Filtering feature is not selected during core configuration.

You can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACL3L4C0R)*/
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
