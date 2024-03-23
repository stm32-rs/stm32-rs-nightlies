#[doc = "Register `DDRCTRL_MRCTRL0` reader"]
pub type R = crate::R<DDRCTRL_MRCTRL0rs>;
#[doc = "Register `DDRCTRL_MRCTRL0` writer"]
pub type W = crate::W<DDRCTRL_MRCTRL0rs>;
#[doc = "Field `MR_TYPE` reader - MR_TYPE"]
pub type MR_TYPE_R = crate::BitReader;
#[doc = "Field `MR_TYPE` writer - MR_TYPE"]
pub type MR_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_RANK` reader - MR_RANK"]
pub type MR_RANK_R = crate::BitReader;
#[doc = "Field `MR_RANK` writer - MR_RANK"]
pub type MR_RANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_ADDR` reader - MR_ADDR"]
pub type MR_ADDR_R = crate::FieldReader;
#[doc = "Field `MR_ADDR` writer - MR_ADDR"]
pub type MR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MR_WR` reader - MR_WR"]
pub type MR_WR_R = crate::BitReader;
#[doc = "Field `MR_WR` writer - MR_WR"]
pub type MR_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    pub fn mr_type(&self) -> MR_TYPE_R {
        MR_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    pub fn mr_rank(&self) -> MR_RANK_R {
        MR_RANK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    pub fn mr_addr(&self) -> MR_ADDR_R {
        MR_ADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    pub fn mr_wr(&self) -> MR_WR_R {
        MR_WR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MR_TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn mr_type(&mut self) -> MR_TYPE_W<DDRCTRL_MRCTRL0rs> {
        MR_TYPE_W::new(self, 0)
    }
    #[doc = "Bit 4 - MR_RANK"]
    #[inline(always)]
    #[must_use]
    pub fn mr_rank(&mut self) -> MR_RANK_W<DDRCTRL_MRCTRL0rs> {
        MR_RANK_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - MR_ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn mr_addr(&mut self) -> MR_ADDR_W<DDRCTRL_MRCTRL0rs> {
        MR_ADDR_W::new(self, 12)
    }
    #[doc = "Bit 31 - MR_WR"]
    #[inline(always)]
    #[must_use]
    pub fn mr_wr(&mut self) -> MR_WR_W<DDRCTRL_MRCTRL0rs> {
        MR_WR_W::new(self, 31)
    }
}
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mrctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_mrctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_MRCTRL0rs;
impl crate::RegisterSpec for DDRCTRL_MRCTRL0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_mrctrl0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_mrctrl0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_MRCTRL0 to value 0x10"]
impl crate::Resettable for DDRCTRL_MRCTRL0rs {
    const RESET_VALUE: u32 = 0x10;
}
