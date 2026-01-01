///Register `C5CR` reader
pub type R = crate::R<C5CRrs>;
///Register `C5CR` writer
pub type W = crate::W<C5CRrs>;
///Field `EN` reader - enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET` writer - reset
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - suspend
pub type SUSP_R = crate::BitReader;
///Field `SUSP` writer - suspend
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - transfer complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIE` reader - half transfer complete interrupt enable
pub type HTIE_R = crate::BitReader;
///Field `HTIE` writer - half transfer complete interrupt enable
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEIE` reader - data transfer error interrupt enable
pub type DTEIE_R = crate::BitReader;
///Field `DTEIE` writer - data transfer error interrupt enable
pub type DTEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULEIE` reader - update link transfer error interrupt enable
pub type ULEIE_R = crate::BitReader;
///Field `ULEIE` writer - update link transfer error interrupt enable
pub type ULEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USEIE` reader - user setting error interrupt enable
pub type USEIE_R = crate::BitReader;
///Field `USEIE` writer - user setting error interrupt enable
pub type USEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPIE` reader - completed suspension interrupt enable
pub type SUSPIE_R = crate::BitReader;
///Field `SUSPIE` writer - completed suspension interrupt enable
pub type SUSPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOIE` reader - trigger overrun interrupt enable
pub type TOIE_R = crate::BitReader;
///Field `TOIE` writer - trigger overrun interrupt enable
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSM` reader - Link step mode
pub type LSM_R = crate::BitReader;
///Field `LSM` writer - Link step mode
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LAP` reader - linked-list allocated port
pub type LAP_R = crate::BitReader;
///Field `LAP` writer - linked-list allocated port
pub type LAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIO` reader - priority level of the channel x GPDMA transfer versus others
pub type PRIO_R = crate::FieldReader;
///Field `PRIO` writer - priority level of the channel x GPDMA transfer versus others
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Link step mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - linked-list allocated port
    #[inline(always)]
    pub fn lap(&self) -> LAP_R {
        LAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 22:23 - priority level of the channel x GPDMA transfer versus others
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C5CR")
            .field("en", &self.en())
            .field("susp", &self.susp())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("dteie", &self.dteie())
            .field("uleie", &self.uleie())
            .field("useie", &self.useie())
            .field("suspie", &self.suspie())
            .field("toie", &self.toie())
            .field("lsm", &self.lsm())
            .field("lap", &self.lap())
            .field("prio", &self.prio())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, C5CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - reset
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, C5CRrs> {
        RESET_W::new(self, 1)
    }
    ///Bit 2 - suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, C5CRrs> {
        SUSP_W::new(self, 2)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, C5CRrs> {
        TCIE_W::new(self, 8)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, C5CRrs> {
        HTIE_W::new(self, 9)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&mut self) -> DTEIE_W<'_, C5CRrs> {
        DTEIE_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&mut self) -> ULEIE_W<'_, C5CRrs> {
        ULEIE_W::new(self, 11)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&mut self) -> USEIE_W<'_, C5CRrs> {
        USEIE_W::new(self, 12)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&mut self) -> SUSPIE_W<'_, C5CRrs> {
        SUSPIE_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<'_, C5CRrs> {
        TOIE_W::new(self, 14)
    }
    ///Bit 16 - Link step mode
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W<'_, C5CRrs> {
        LSM_W::new(self, 16)
    }
    ///Bit 17 - linked-list allocated port
    #[inline(always)]
    pub fn lap(&mut self) -> LAP_W<'_, C5CRrs> {
        LAP_W::new(self, 17)
    }
    ///Bits 22:23 - priority level of the channel x GPDMA transfer versus others
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<'_, C5CRrs> {
        PRIO_W::new(self, 22)
    }
}
/**GPDMA channel 5 control register

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPDMA:C5CR)*/
pub struct C5CRrs;
impl crate::RegisterSpec for C5CRrs {
    type Ux = u32;
}
///`read()` method returns [`c5cr::R`](R) reader structure
impl crate::Readable for C5CRrs {}
///`write(|w| ..)` method takes [`c5cr::W`](W) writer structure
impl crate::Writable for C5CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C5CR to value 0
impl crate::Resettable for C5CRrs {}
