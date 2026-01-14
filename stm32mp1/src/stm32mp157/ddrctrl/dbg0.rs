///Register `DBG0` reader
pub type R = crate::R<DBG0rs>;
///Register `DBG0` writer
pub type W = crate::W<DBG0rs>;
///Field `DIS_WC` reader - DIS_WC
pub type DIS_WC_R = crate::BitReader;
///Field `DIS_WC` writer - DIS_WC
pub type DIS_WC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_COLLISION_PAGE_OPT` reader - DIS_COLLISION_PAGE_OPT
pub type DIS_COLLISION_PAGE_OPT_R = crate::BitReader;
///Field `DIS_COLLISION_PAGE_OPT` writer - DIS_COLLISION_PAGE_OPT
pub type DIS_COLLISION_PAGE_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DIS_WC
    #[inline(always)]
    pub fn dis_wc(&self) -> DIS_WC_R {
        DIS_WC_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DIS_COLLISION_PAGE_OPT
    #[inline(always)]
    pub fn dis_collision_page_opt(&self) -> DIS_COLLISION_PAGE_OPT_R {
        DIS_COLLISION_PAGE_OPT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG0")
            .field("dis_wc", &self.dis_wc())
            .field("dis_collision_page_opt", &self.dis_collision_page_opt())
            .finish()
    }
}
impl W {
    ///Bit 0 - DIS_WC
    #[inline(always)]
    pub fn dis_wc(&mut self) -> DIS_WC_W<'_, DBG0rs> {
        DIS_WC_W::new(self, 0)
    }
    ///Bit 4 - DIS_COLLISION_PAGE_OPT
    #[inline(always)]
    pub fn dis_collision_page_opt(&mut self) -> DIS_COLLISION_PAGE_OPT_W<'_, DBG0rs> {
        DIS_COLLISION_PAGE_OPT_W::new(self, 4)
    }
}
/**DDRCTRL debug register 0

You can [`read`](crate::Reg::read) this register and get [`dbg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DBG0)*/
pub struct DBG0rs;
impl crate::RegisterSpec for DBG0rs {
    type Ux = u32;
}
///`read()` method returns [`dbg0::R`](R) reader structure
impl crate::Readable for DBG0rs {}
///`write(|w| ..)` method takes [`dbg0::W`](W) writer structure
impl crate::Writable for DBG0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG0 to value 0
impl crate::Resettable for DBG0rs {}
