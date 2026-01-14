///Register `K3LR` writer
pub type W = crate::W<K3LRrs>;
///Field `b32` writer - b32
pub type B32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b33` writer - b33
pub type B33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b34` writer - b34
pub type B34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b35` writer - b35
pub type B35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b36` writer - b36
pub type B36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b37` writer - b37
pub type B37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b38` writer - b38
pub type B38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b39` writer - b39
pub type B39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b40` writer - b40
pub type B40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b41` writer - b41
pub type B41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b42` writer - b42
pub type B42_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b43` writer - b43
pub type B43_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b44` writer - b44
pub type B44_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b45` writer - b45
pub type B45_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b46` writer - b46
pub type B46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b47` writer - b47
pub type B47_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b48` writer - b48
pub type B48_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b49` writer - b49
pub type B49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b50` writer - b50
pub type B50_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b51` writer - b51
pub type B51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b52` writer - b52
pub type B52_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b53` writer - b53
pub type B53_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b54` writer - b54
pub type B54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b55` writer - b55
pub type B55_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b56` writer - b56
pub type B56_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b57` writer - b57
pub type B57_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b58` writer - b58
pub type B58_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b59` writer - b59
pub type B59_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b60` writer - b60
pub type B60_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b61` writer - b61
pub type B61_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b62` writer - b62
pub type B62_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b63` writer - b63
pub type B63_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<K3LRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - b32
    #[inline(always)]
    pub fn b32(&mut self) -> B32_W<'_, K3LRrs> {
        B32_W::new(self, 0)
    }
    ///Bit 1 - b33
    #[inline(always)]
    pub fn b33(&mut self) -> B33_W<'_, K3LRrs> {
        B33_W::new(self, 1)
    }
    ///Bit 2 - b34
    #[inline(always)]
    pub fn b34(&mut self) -> B34_W<'_, K3LRrs> {
        B34_W::new(self, 2)
    }
    ///Bit 3 - b35
    #[inline(always)]
    pub fn b35(&mut self) -> B35_W<'_, K3LRrs> {
        B35_W::new(self, 3)
    }
    ///Bit 4 - b36
    #[inline(always)]
    pub fn b36(&mut self) -> B36_W<'_, K3LRrs> {
        B36_W::new(self, 4)
    }
    ///Bit 5 - b37
    #[inline(always)]
    pub fn b37(&mut self) -> B37_W<'_, K3LRrs> {
        B37_W::new(self, 5)
    }
    ///Bit 6 - b38
    #[inline(always)]
    pub fn b38(&mut self) -> B38_W<'_, K3LRrs> {
        B38_W::new(self, 6)
    }
    ///Bit 7 - b39
    #[inline(always)]
    pub fn b39(&mut self) -> B39_W<'_, K3LRrs> {
        B39_W::new(self, 7)
    }
    ///Bit 8 - b40
    #[inline(always)]
    pub fn b40(&mut self) -> B40_W<'_, K3LRrs> {
        B40_W::new(self, 8)
    }
    ///Bit 9 - b41
    #[inline(always)]
    pub fn b41(&mut self) -> B41_W<'_, K3LRrs> {
        B41_W::new(self, 9)
    }
    ///Bit 10 - b42
    #[inline(always)]
    pub fn b42(&mut self) -> B42_W<'_, K3LRrs> {
        B42_W::new(self, 10)
    }
    ///Bit 11 - b43
    #[inline(always)]
    pub fn b43(&mut self) -> B43_W<'_, K3LRrs> {
        B43_W::new(self, 11)
    }
    ///Bit 12 - b44
    #[inline(always)]
    pub fn b44(&mut self) -> B44_W<'_, K3LRrs> {
        B44_W::new(self, 12)
    }
    ///Bit 13 - b45
    #[inline(always)]
    pub fn b45(&mut self) -> B45_W<'_, K3LRrs> {
        B45_W::new(self, 13)
    }
    ///Bit 14 - b46
    #[inline(always)]
    pub fn b46(&mut self) -> B46_W<'_, K3LRrs> {
        B46_W::new(self, 14)
    }
    ///Bit 15 - b47
    #[inline(always)]
    pub fn b47(&mut self) -> B47_W<'_, K3LRrs> {
        B47_W::new(self, 15)
    }
    ///Bit 16 - b48
    #[inline(always)]
    pub fn b48(&mut self) -> B48_W<'_, K3LRrs> {
        B48_W::new(self, 16)
    }
    ///Bit 17 - b49
    #[inline(always)]
    pub fn b49(&mut self) -> B49_W<'_, K3LRrs> {
        B49_W::new(self, 17)
    }
    ///Bit 18 - b50
    #[inline(always)]
    pub fn b50(&mut self) -> B50_W<'_, K3LRrs> {
        B50_W::new(self, 18)
    }
    ///Bit 19 - b51
    #[inline(always)]
    pub fn b51(&mut self) -> B51_W<'_, K3LRrs> {
        B51_W::new(self, 19)
    }
    ///Bit 20 - b52
    #[inline(always)]
    pub fn b52(&mut self) -> B52_W<'_, K3LRrs> {
        B52_W::new(self, 20)
    }
    ///Bit 21 - b53
    #[inline(always)]
    pub fn b53(&mut self) -> B53_W<'_, K3LRrs> {
        B53_W::new(self, 21)
    }
    ///Bit 22 - b54
    #[inline(always)]
    pub fn b54(&mut self) -> B54_W<'_, K3LRrs> {
        B54_W::new(self, 22)
    }
    ///Bit 23 - b55
    #[inline(always)]
    pub fn b55(&mut self) -> B55_W<'_, K3LRrs> {
        B55_W::new(self, 23)
    }
    ///Bit 24 - b56
    #[inline(always)]
    pub fn b56(&mut self) -> B56_W<'_, K3LRrs> {
        B56_W::new(self, 24)
    }
    ///Bit 25 - b57
    #[inline(always)]
    pub fn b57(&mut self) -> B57_W<'_, K3LRrs> {
        B57_W::new(self, 25)
    }
    ///Bit 26 - b58
    #[inline(always)]
    pub fn b58(&mut self) -> B58_W<'_, K3LRrs> {
        B58_W::new(self, 26)
    }
    ///Bit 27 - b59
    #[inline(always)]
    pub fn b59(&mut self) -> B59_W<'_, K3LRrs> {
        B59_W::new(self, 27)
    }
    ///Bit 28 - b60
    #[inline(always)]
    pub fn b60(&mut self) -> B60_W<'_, K3LRrs> {
        B60_W::new(self, 28)
    }
    ///Bit 29 - b61
    #[inline(always)]
    pub fn b61(&mut self) -> B61_W<'_, K3LRrs> {
        B61_W::new(self, 29)
    }
    ///Bit 30 - b62
    #[inline(always)]
    pub fn b62(&mut self) -> B62_W<'_, K3LRrs> {
        B62_W::new(self, 30)
    }
    ///Bit 31 - b63
    #[inline(always)]
    pub fn b63(&mut self) -> B63_W<'_, K3LRrs> {
        B63_W::new(self, 31)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#CRYP:K3LR)*/
pub struct K3LRrs;
impl crate::RegisterSpec for K3LRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k3lr::W`](W) writer structure
impl crate::Writable for K3LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K3LR to value 0
impl crate::Resettable for K3LRrs {}
